use std::collections::HashMap;
use std::fs;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::times_ui::{TimesUI, UIRequest, UIResponse};
use super::{AppRequest, AppResponse, Model, State};
use serde::Serialize;

use timesman_bstore::{PostStore, TimesStore, TodoStore};
use timesman_type::{Post, Tag, TagId, Times, Todo};
use tokio::runtime::Runtime;

#[derive(Debug)]
enum AsyncEvent {
    AddPost(Post),
    UpdatePost(Post),
    AddTodo(Todo),
    AddTag(Tag),
    UpdateTodo(Todo),
    Err(String),
}

pub struct TimesModel {
    ui: TimesUI,
    tstore: Arc<Mutex<dyn TimesStore>>,
    pstore: Arc<Mutex<dyn PostStore>>,
    posts: Vec<Post>,
    tdstore: Arc<Mutex<dyn TodoStore>>,
    todos: Vec<Todo>,
    tags: HashMap<TagId, Tag>,
    aetx: Sender<AsyncEvent>,
    aerx: Receiver<AsyncEvent>,
    urtx: Sender<UIResponse>,
    urrx: Receiver<UIResponse>,
    sort_reverse: bool,
}

async fn load_posts(
    pstore: Arc<Mutex<dyn PostStore>>,
    tx: &Sender<AsyncEvent>,
) {
    let mut pstore = pstore.lock().await;

    let posts = pstore.get_all().await.unwrap();

    for post in posts {
        tx.send(AsyncEvent::AddPost(post.clone())).unwrap();
    }
}

async fn load_tags(pstore: Arc<Mutex<dyn PostStore>>, tx: &Sender<AsyncEvent>) {
    let mut pstore = pstore.lock().await;
    let tags = pstore.get_tags().await.unwrap();

    for tag in tags {
        tx.send(AsyncEvent::AddTag(tag.clone())).unwrap();
    }
}

fn todo_setup(
    tstore: Arc<Mutex<dyn TimesStore>>,
    tx: Sender<AsyncEvent>,
    rt: &Runtime,
) -> Arc<Mutex<dyn TodoStore>> {
    let tdstore = {
        rt.block_on(async move {
            let mut tstore = tstore.lock().await;
            let tdstore = tstore.tdstore().await.unwrap();
            tdstore
        })
    };

    {
        let tdstore = tdstore.clone();

        rt.spawn(async move {
            let mut tdstore = tdstore.lock().await;

            let todos = tdstore.get().await.unwrap();

            for todo in todos {
                tx.send(AsyncEvent::AddTodo(todo.clone())).unwrap();
            }
        });
    }

    tdstore
}

impl TimesModel {
    pub fn new(tstore: Arc<Mutex<dyn TimesStore>>, rt: &Runtime) -> Self {
        let (aetx, aerx) = channel();

        let pstore = {
            let tstore = tstore.clone();

            rt.block_on(async move {
                let mut tstore = tstore.lock().await;
                let pstore = tstore.pstore().await.unwrap();
                pstore
            })
        };

        {
            let pstore = pstore.clone();
            let tx = aetx.clone();

            rt.spawn(async move { load_posts(pstore, &tx).await });
        }
        {
            let pstore = pstore.clone();
            let tx = aetx.clone();
            rt.spawn(async move { load_tags(pstore, &tx).await });
        }

        let tdstore = todo_setup(tstore.clone(), aetx.clone(), rt);

        let (urtx, urrx) = channel();

        let times = {
            let tstore = tstore.clone();
            rt.block_on(async move {
                let mut tstore = tstore.lock().await;
                tstore.get().await.unwrap()
            })
        };

        let ui = TimesUI::new(times.title);

        Self {
            ui,
            tstore,
            pstore,
            posts: vec![],
            tdstore,
            todos: vec![],
            tags: HashMap::new(),
            aetx,
            aerx,
            urtx,
            urrx,
            sort_reverse: false,
        }
    }

    fn sort_post(&mut self) {
        if self.sort_reverse {
            self.posts.sort_by(|a, b| a.id.cmp(&b.id).reverse());
        } else {
            self.posts.sort_by(|a, b| a.id.cmp(&b.id));
        }
    }

    fn handle_ureqs(
        &mut self,
        ureq: Vec<UIRequest>,
        areq: &mut Vec<AppRequest>,
        rt: &Runtime,
    ) {
        for req in ureq {
            println!("{:?}", req);
            match req {
                UIRequest::Post(content, file) => {
                    let pstore = self.pstore.clone();
                    let aetx = self.aetx.clone();
                    let urtx = self.urtx.clone();
                    rt.spawn(async move {
                        let mut pstore = pstore.lock().await;
                        let post = pstore
                            .post(content.clone(), file.clone())
                            .await
                            .unwrap();
                        aetx.send(AsyncEvent::AddPost(post)).unwrap();
                        urtx.send(UIResponse::ClearText).unwrap();
                    });
                }
                UIRequest::UpdatePost(post) => {
                    let pstore = self.pstore.clone();
                    let aetx = self.aetx.clone();
                    rt.spawn(async move {
                        let mut pstore = pstore.lock().await;
                        let post = pstore.update(post).await.unwrap();
                        aetx.send(AsyncEvent::UpdatePost(post)).unwrap();
                    });
                }
                UIRequest::Tag(name) => {
                    let pstore = self.pstore.clone();
                    let aetx = self.aetx.clone();
                    let urtx = self.urtx.clone();

                    rt.spawn(async move {
                        let mut pstore = pstore.lock().await;
                        let tag = pstore.create_tag(name).await.unwrap();
                        aetx.send(AsyncEvent::AddTag(tag)).unwrap();
                        urtx.send(UIResponse::ClearTextSidePane).unwrap();
                    });
                }
                UIRequest::Todo(todo) => {
                    let tdstore = self.tdstore.clone();
                    let pstore = self.pstore.clone();
                    let aetx = self.aetx.clone();
                    let urtx = self.urtx.clone();
                    rt.spawn(async move {
                        let mut tdstore = tdstore.lock().await;
                        match tdstore.new(todo.clone()).await {
                            Ok(todo) => {
                                let mut pstore = pstore.lock().await;
                                match pstore
                                    .post(
                                        format!("todo ({}) is created", todo.content),
                                        None,
                                    )
                                    .await {
                                    Ok(post) => {
                                        aetx.send(AsyncEvent::AddTodo(todo)).unwrap();
                                        aetx.send(AsyncEvent::AddPost(post)).unwrap();
                                        urtx.send(UIResponse::ClearTextSidePane).unwrap();
                                    }
                                    Err(e) => {
                                        aetx.send(AsyncEvent::Err(format!("Failed to create post: {}", e))).unwrap();
                                    }
                                }
                            }
                            Err(e) => {
                                aetx.send(AsyncEvent::Err(format!("Failed to create todo: {}", e))).unwrap();
                            }
                        }
                    });
                }
                UIRequest::TodoWithDetail(content, detail) => {
                    let tdstore = self.tdstore.clone();
                    let pstore = self.pstore.clone();
                    let aetx = self.aetx.clone();
                    let urtx = self.urtx.clone();
                    rt.spawn(async move {
                        let mut tdstore = tdstore.lock().await;
                        // Create a regular todo first, then update it with detail
                        match tdstore.new(content.clone()).await {
                            Ok(mut todo) => {
                                todo.detail = Some(detail.clone());
                                match tdstore.update(todo).await {
                                    Ok(todo) => {
                                        let mut pstore = pstore.lock().await;
                                        let post_content = if detail.is_empty() {
                                            format!("todo ({}) is created", content)
                                        } else {
                                            format!("todo ({}) is created with detail", content)
                                        };
                                        match pstore.post(post_content, None).await {
                                            Ok(post) => {
                                                aetx.send(AsyncEvent::AddTodo(todo)).unwrap();
                                                aetx.send(AsyncEvent::AddPost(post)).unwrap();
                                                urtx.send(UIResponse::ClearTextSidePane).unwrap();
                                                urtx.send(UIResponse::ClearTextSidePaneDetail).unwrap();
                                            }
                                            Err(e) => {
                                                aetx.send(AsyncEvent::Err(format!("Failed to create post: {}", e))).unwrap();
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        aetx.send(AsyncEvent::Err(format!("Failed to update todo with detail: {}", e))).unwrap();
                                    }
                                }
                            }
                            Err(e) => {
                                aetx.send(AsyncEvent::Err(format!("Failed to create todo: {}", e))).unwrap();
                            }
                        }
                    });
                }
                UIRequest::UpdateTodoDetail(tdid, detail) => {
                    let tdstore = self.tdstore.clone();
                    let aetx = self.aetx.clone();
                    let todos = self.todos.clone();
                    rt.spawn(async move {
                        let mut tdstore = tdstore.lock().await;
                        // Find the existing todo and update its detail
                        if let Some(mut todo) = todos.into_iter().find(|t| t.id == tdid) {
                            todo.detail = Some(detail.clone());
                            match tdstore.update(todo).await {
                                Ok(updated_todo) => {
                                    aetx.send(AsyncEvent::UpdateTodo(updated_todo)).unwrap();
                                }
                                Err(e) => {
                                    aetx.send(AsyncEvent::Err(format!("Failed to update todo detail: {}", e))).unwrap();
                                }
                            }
                        } else {
                            aetx.send(AsyncEvent::Err(format!("Todo with ID {} not found", tdid))).unwrap();
                        }
                    });
                }
                UIRequest::TodoDone(tdid, done) => {
                    let tdstore = self.tdstore.clone();
                    let pstore = self.pstore.clone();
                    let aetx = self.aetx.clone();

                    rt.spawn(async move {
                        let mut tdstore = tdstore.lock().await;
                        match tdstore.done(tdid, done).await {
                            Ok(todo) => {
                                let mut pstore = pstore.lock().await;
                                let status = if done { "done" } else { "undone" };
                                match pstore
                                    .post(
                                        format!("todo ({}) is {}", &todo.content, status),
                                        None,
                                    )
                                    .await {
                                    Ok(post) => {
                                        aetx.send(AsyncEvent::UpdateTodo(todo)).unwrap();
                                        aetx.send(AsyncEvent::AddPost(post)).unwrap();
                                    }
                                    Err(e) => {
                                        aetx.send(AsyncEvent::Err(format!("Failed to create post for todo status: {}", e))).unwrap();
                                    }
                                }
                            }
                            Err(e) => {
                                aetx.send(AsyncEvent::Err(format!("Failed to update todo status: {}", e))).unwrap();
                            }
                        }
                    });
                }
                UIRequest::Dump(path) => {
                    let tstore = self.tstore.clone();
                    let posts = self.posts.clone();
                    let tags: Vec<Tag> =
                        self.tags.iter().fold(vec![], |mut a, t| {
                            a.push(t.1.clone());
                            a
                        });
                    let mut file = fs::File::create(path).unwrap();
                    rt.spawn(async move {
                        let mut tstore = tstore.lock().await;
                        let times = tstore.get().await.unwrap();
                        let dump = DumpData::build(times, posts, tags);
                        serde_json::to_writer(&mut file, &dump).unwrap();
                    });
                }
                UIRequest::Sort(reverse) => {
                    self.sort_reverse = reverse;
                    self.sort_post();
                }
                UIRequest::Close => {
                    areq.push(AppRequest::ChangeState(State::Back));
                }
            }
        }
    }

    fn handle_async_event(&mut self, areq: &mut Vec<AppRequest>) {
        loop {
            match self.aerx.try_recv() {
                Ok(event) => match event {
                    AsyncEvent::AddPost(post) => {
                        self.posts.push(post);
                        self.sort_post();
                    }
                    AsyncEvent::UpdatePost(post) => {
                        self.posts.iter_mut().for_each(|p| {
                            if p.id == post.id {
                                *p = post.clone();
                            }
                        });
                    }
                    AsyncEvent::AddTodo(todo) => {
                        self.todos.push(todo);
                    }
                    AsyncEvent::AddTag(tag) => {
                        self.tags.insert(tag.id, tag);
                    }
                    AsyncEvent::Err(e) => {
                        areq.push(AppRequest::Err(e));
                    }
                    AsyncEvent::UpdateTodo(todo) => {
                        self.todos
                            .iter_mut()
                            .filter(|t| t.id == todo.id)
                            .for_each(|t| *t = todo.clone());
                    }
                },
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(e) => {
                    areq.push(AppRequest::Err(format!("{e}")));
                }
            }
        }
    }

    fn gen_ures_vec(&self) -> Vec<UIResponse> {
        let mut ures = vec![];
        loop {
            match self.urrx.try_recv() {
                Ok(res) => {
                    ures.push(res);
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(_e) => {
                    todo!();
                }
            }
        }

        ures
    }

    fn handle_app_resp(
        &self,
        resp: &Vec<AppResponse>,
        ures: &mut Vec<UIResponse>,
    ) {
        for r in resp {
            match r {
                AppResponse::FileDropped(path) => {
                    ures.push(UIResponse::FileDropped(path.clone()));
                }
                AppResponse::Error(_) => {
                    // Errors are not relevant for times model
                }
            }
        }
    }
}

impl Model for TimesModel {
    fn update(
        &mut self,
        ctx: &egui::Context,
        rt: &Runtime,
        resp: Vec<AppResponse>,
    ) -> Result<Vec<AppRequest>, String> {
        let mut areqs = vec![];

        let mut ures = self.gen_ures_vec();
        self.handle_app_resp(&resp, &mut ures);

        let ureqs =
            self.ui
                .update(ctx, &self.posts, &self.todos, &self.tags, ures);

        self.handle_ureqs(ureqs, &mut areqs, rt);
        self.handle_async_event(&mut areqs);

        Ok(areqs)
    }
}

#[derive(Serialize)]
struct DumpData {
    times: Times,
    posts: Vec<Post>,
    tags: Vec<Tag>,
}

impl DumpData {
    pub fn build(times: Times, posts: Vec<Post>, tags: Vec<Tag>) -> Self {
        let mut modified_post = posts.clone();
        modified_post.iter_mut().for_each(|p| p.file = None);

        Self {
            times,
            posts: modified_post,
            tags,
        }
    }
}
