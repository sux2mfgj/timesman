use super::serde_json;
use super::PostStore;
use super::{async_trait, Arc, Mutex, UnQLite, KV};
use super::{File, Pid, Post, Tag, TagId, Tid};
use serde::{Deserialize, Serialize};

pub struct LocalPostStore {
    tid: Tid,
    store: Arc<Mutex<UnQLite>>,
    pmeta: PostMeta,
    tag_meta: TagMeta,
}

fn get_pmeta_path(tid: Tid) -> String {
    format!("{}/posts/meta.data", tid)
}

fn get_post_path(tid: Tid, pid: Pid) -> String {
    format!("{tid}/posts/{pid}")
}

fn get_tag_meta_path(tid: Tid) -> String {
    format!("{tid}/tags/meta.data")
}

fn get_tag_path(tid: Tid, tagid: TagId) -> String {
    format!("{tid}/tags/{tagid}")
}

impl LocalPostStore {
    async fn load_pmeta(
        tid: Tid,
        store: Arc<Mutex<UnQLite>>,
    ) -> Result<PostMeta, String> {
        let store = store.lock().await;
        let meta_path = get_pmeta_path(tid);

        let meta = if !store.kv_contains(&meta_path) {
            let meta = PostMeta::default();
            let data = serde_json::to_string(&meta).unwrap();
            store.kv_store(&meta_path, data.into_bytes()).unwrap();
            meta
        } else {
            let data = store.kv_fetch(&meta_path).unwrap();
            serde_json::from_slice(&data).unwrap()
        };

        Ok(meta)
    }

    async fn load_tag_meta(
        tid: Tid,
        store: Arc<Mutex<UnQLite>>,
    ) -> Result<TagMeta, String> {
        let meta_path = get_tag_meta_path(tid);

        let store = store.lock().await;

        let tag_meta = if !store.kv_contains(&meta_path) {
            let meta = TagMeta::default();
            let data = serde_json::to_string(&meta).unwrap();
            store.kv_store(&meta_path, data.into_bytes()).unwrap();
            meta
        } else {
            let data = store.kv_fetch(&meta_path).unwrap();
            serde_json::from_slice(&data).unwrap()
        };

        Ok(tag_meta)
    }

    pub async fn new(tid: Tid, store: Arc<Mutex<UnQLite>>) -> Self {
        let pmeta = Self::load_pmeta(tid, store.clone()).await.unwrap();
        let tag_meta = Self::load_tag_meta(tid, store.clone()).await.unwrap();

        Self {
            tid,
            store,
            pmeta,
            tag_meta,
        }
    }

    async fn sync_post_meta(&self) {
        let data = serde_json::to_string(&self.pmeta).unwrap();

        let store = self.store.lock().await;
        store
            .kv_store(get_pmeta_path(self.tid), data.into_bytes())
            .unwrap();
    }

    async fn sync_tag_meta(&self) {
        let data = serde_json::to_string(&self.tag_meta).unwrap();

        let store = self.store.lock().await;
        store
            .kv_store(get_tag_meta_path(self.tid), data.into_bytes())
            .unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct PostMeta {
    npid: Pid,
    pids: Vec<Pid>,
}

impl PostMeta {
    pub fn append(&mut self, pid: Pid) {
        self.pids.push(pid);
        // npid should always be max(existing_ids) + 1 to avoid collisions
        self.npid = self.pids.iter().max().map(|x| x + 1).unwrap_or(0);
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TagMeta {
    ntagid: TagId,
    tagids: Vec<TagId>,
}

impl TagMeta {
    pub fn append(&mut self, tagid: TagId) {
        self.tagids.push(tagid);
        self.ntagid += 1;
    }
}

#[async_trait]
impl PostStore for LocalPostStore {
    async fn get(&mut self, _pid: Pid) -> Result<Post, String> {
        todo!();
    }

    async fn get_all(&mut self) -> Result<Vec<Post>, String> {
        let store = self.store.lock().await;
        let mut posts = vec![];
        for pid in &self.pmeta.pids {
            let data = store.kv_fetch(get_post_path(self.tid, *pid)).unwrap();
            let post: Post = serde_json::from_slice(&data).unwrap();
            posts.push(post);
        }

        Ok(posts)
    }

    async fn get_tags(&mut self) -> Result<Vec<Tag>, String> {
        let store = self.store.lock().await;

        let mut tags = vec![];
        for tagid in &self.tag_meta.tagids {
            let data = store.kv_fetch(get_tag_path(self.tid, *tagid)).unwrap();
            let tag: Tag = serde_json::from_slice(&data).unwrap();
            tags.push(tag)
        }

        Ok(tags)
    }

    async fn create_tag(&mut self, name: String) -> Result<Tag, String> {
        let id = self.tag_meta.ntagid;

        let tag = Tag { id, name };

        {
            let text = serde_json::to_string(&tag).unwrap();
            let store = self.store.lock().await;
            store
                .kv_store(get_tag_path(self.tid, tag.id), text.into_bytes())
                .unwrap();
        }

        self.tag_meta.append(tag.id);

        self.sync_tag_meta().await;

        Ok(tag)
    }

    async fn post(
        &mut self,
        post: String,
        file: Option<File>,
    ) -> Result<Post, String> {
        // Get next available ID and increment atomically
        let pid = self.pmeta.npid;
        self.pmeta.npid += 1;

        let post = Post {
            id: pid,
            post,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: None,
            file,
            tag: None,
        };

        let text = serde_json::to_string(&post)
            .map_err(|e| format!("Failed to serialize post: {}", e))?;
        
        // add a scope to avoid deadlock
        {
            let store = self.store.lock().await;
            store
                .kv_store(get_post_path(self.tid, pid), text.into_bytes())
                .map_err(|e| format!("Failed to store post: {}", e))?;
        }

        // Add to metadata list
        self.pmeta.pids.push(pid);

        self.sync_post_meta().await;

        Ok(post)
    }

    async fn delete(&mut self, _pid: Pid) -> Result<(), String> {
        todo!();
    }

    async fn update(&mut self, post: Post) -> Result<Post, String> {
        let text = serde_json::to_string(&post).unwrap();
        {
            let store = self.store.lock().await;
            store
                .kv_store(get_post_path(self.tid, post.id), text.into_bytes())
                .unwrap();
        }

        Ok(post)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_consistency() {
        let tid = 123;
        let pid = 456;
        let tagid = 789;

        // Test post paths
        assert_eq!(get_pmeta_path(tid), "123/posts/meta.data");
        assert_eq!(get_post_path(tid, pid), "123/posts/456");
        
        // Test tag paths (should be plural and consistent)
        assert_eq!(get_tag_meta_path(tid), "123/tags/meta.data");
        assert_eq!(get_tag_path(tid, tagid), "123/tags/789");
        
        // Verify no dollar signs in paths
        assert!(!get_post_path(tid, pid).contains('$'));
        assert!(!get_tag_path(tid, tagid).contains('$'));
    }
}
