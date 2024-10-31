use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CommentCreate {
    pub guest: String,
    pub article_id: i32,
    pub parent_id: Option<i32>,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CommentUpdate {
    pub id: i32,
    pub comment: String,
    pub like_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Comment {
    pub id: i32,
    pub guest: String,
    pub article_id: i32,
    pub parent_id: Option<i32>, // 父评论 ID，如果是顶级评论则为 None
    pub comment: String,        // 评论内容
    pub created_at: chrono::DateTime<Utc>, // 评论时间
    pub depth: i32,             // 评论层级
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CommentsDisplay {
    pub id: i32,
    pub guest: String,
    pub article_id: i32,
    pub parent_id: Option<i32>,
    pub comment: String,
    pub created_at: chrono::DateTime<Utc>,
    pub depth: i32,
    pub child: Vec<CommentsDisplay>, // 子评论
}

impl CommentsDisplay {
    pub fn new_top_comment(comment: Comment) -> Self {
        CommentsDisplay {
            id: comment.id,
            guest: comment.guest,
            article_id: comment.article_id,
            parent_id: comment.parent_id,
            comment: comment.comment,
            created_at: comment.created_at,
            // like_count: comment.like_count,
            depth: comment.depth,
            child: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: CommentsDisplay) {
        self.child.push(child);
    }
}
// 递归查找并添加子评论
pub fn add_comment_to_parent(
    comment: CommentsDisplay,
    comments_display: &mut Vec<CommentsDisplay>,
) {
    for parent in comments_display.iter_mut() {
        if parent.id == comment.parent_id.unwrap_or(-1) {
            parent.add_child(comment); // 添加为子评论
            return;
        }

        // 递归检查子评论
        add_comment_to_parent(comment.clone(), &mut parent.child);
    }
}
