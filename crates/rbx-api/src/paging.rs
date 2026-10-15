//! Pagination helpers for both styles Roblox uses.

use std::future::Future;

use crate::error::Result;

/// One page in the cursor style (`data`, `nextPageCursor`).
#[derive(Debug, Clone)]
pub struct CursorPage<T> {
    pub items: Vec<T>,
    pub next: Option<String>,
}

/// Incremental cursor pager for lists the UI loads as the user scrolls.
#[derive(Debug, Clone, Default)]
pub struct Cursor {
    next: Option<String>,
    started: bool,
    done: bool,
}

impl Cursor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn done(&self) -> bool {
        self.done
    }
    /// Fetch the next page, or `None` when the list is exhausted.
    pub async fn next<T, F, Fut>(&mut self, fetch: F) -> Result<Option<Vec<T>>>
    where
        F: FnOnce(Option<String>) -> Fut,
        Fut: Future<Output = Result<CursorPage<T>>>,
    {
        if self.done {
            return Ok(None);
        }
        let page = fetch(if self.started { self.next.clone() } else { None }).await?;
        self.started = true;
        self.next = page.next.filter(|c| !c.is_empty());
        self.done = self.next.is_none();
        Ok(Some(page.items))
    }
}

/// Read every page of a cursor list, up to `max_pages`.
pub async fn collect_cursor<T, F, Fut>(mut fetch: F, max_pages: usize) -> Result<Vec<T>>
where
    F: FnMut(Option<String>) -> Fut,
    Fut: Future<Output = Result<CursorPage<T>>>,
{
    let mut out = Vec::new();
    let mut c = Cursor::new();
    for _ in 0..max_pages {
        match c.next(&mut fetch).await? {
            Some(items) => out.extend(items),
            None => break,
        }
        if c.done() {
            break;
        }
    }
    Ok(out)
}

/// Read every page of a page-number list (1-based), stopping at a short or empty page.
pub async fn collect_pages<T, F, Fut>(mut fetch: F, page_size: usize, max_pages: usize) -> Result<Vec<T>>
where
    F: FnMut(u32) -> Fut,
    Fut: Future<Output = Result<Vec<T>>>,
{
    let mut out = Vec::new();
    for page in 1..=max_pages as u32 {
        let items = fetch(page).await?;
        let n = items.len();
        out.extend(items);
        if n < page_size || n == 0 {
            break;
        }
    }
    Ok(out)
}
