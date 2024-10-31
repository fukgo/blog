import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { getArticleCommentUrl, postArticleCommentUrl } from '@/api_list';

const CommentSection = ({ articleId }) => {
    const [comments, setComments] = useState([]);
    const [newComment, setNewComment] = useState('');
    const [username, setUsername] = useState('');
    const [replyParentId, setReplyParentId] = useState(null);
    const [showReplyForm, setShowReplyForm] = useState(false);
    const [replyToUser, setReplyToUser] = useState('');

    useEffect(() => {
        const fetchComments = async () => {
            try {
                const response = await axios.get(getArticleCommentUrl(articleId));
                setComments(response.data);
            } catch (error) {
            }
        };

        fetchComments();
    }, [articleId]);

    const handleCommentSubmit = async (parentId = null) => {
        if (!username || !newComment) return;
        
        try {
            await axios.post(postArticleCommentUrl(), {
                guest: username || 'Anonymous', // Use entered username or default to 'Anonymous'
                article_id: articleId,
                comment: newComment,
                parent_id: parentId,
            });
            setNewComment('');
            setShowReplyForm(false);
            setReplyParentId(null);
            setReplyToUser('');
            // Reload comments
            const response = await axios.get(getArticleCommentUrl(articleId));
            setComments(response.data);
        } catch (error) {
        }
    };

    const renderComments = (comments, depth = 0) => {
        return comments.map(comment => (
            <div key={comment.id} className="ml-4 p-4 bg-white rounded-lg shadow mb-4" style={{ marginLeft: depth * 20 }}>
                <div className="flex justify-between items-center">
                    <span className="font-bold">{comment.guest} 说：</span>
                    {comment.child && comment.child.length > 0 && (
                        <button
                            className="bg-blue-500 text-white px-2 py-1 rounded hover:bg-blue-700"
                            onClick={() => {
                                const childContainer = document.getElementById(`child-comments-${comment.id}`);
                                if (childContainer.style.display === 'none') {
                                    childContainer.style.display = 'block';
                                } else {
                                    childContainer.style.display = 'none';
                                }
                            }}
                        >
                            展开
                        </button>
                    )}
                </div>
                <div className="mt-2">{comment.comment}</div>
                <div className="text-sm text-gray-500 mt-2">
                    <small>{new Date(comment.created_at).toLocaleString()}</small>
                </div>
                <button
                    className="mt-2 bg-green-500 text-white px-2 py-1 rounded hover:bg-green-700"
                    onClick={() => {
                        setReplyParentId(comment.id);
                        setReplyToUser(comment.guest);
                        setShowReplyForm(true);
                    }}
                >
                    回复
                </button>
                <div id={`child-comments-${comment.id}`} className="child-comments mt-4" style={{ display: 'none' }}>
                    {comment.child && renderComments(comment.child, depth + 1)}
                </div>
            </div>
        ));
    };

    return (
        <div>
            <div id="comments-container">
                {renderComments(comments)}
            </div>
            {showReplyForm && (
                <div id="reply-form-container" className="reply-form mt-4 p-4 bg-gray-100 rounded-lg shadow">
                    <p className="mb-2">回复 {replyToUser}:</p>
                    <input
                        type="text"
                        id="reply-username"
                        placeholder="输入你的名字"
                        value={username}
                        onChange={(e) => setUsername(e.target.value)}
                        className="mb-2 p-2 border rounded w-full"
                    />
                    <textarea
                        id="reply-comment"
                        placeholder="输入你的评论"
                        value={newComment}
                        onChange={(e) => setNewComment(e.target.value)}
                        required
                        className="mb-2 p-2 border rounded w-full"
                    />
                    <div className="flex justify-end">
                        <button
                            className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-700 mr-2"
                            onClick={() => handleCommentSubmit(replyParentId)}
                            disabled={!username || !newComment} // Disable if fields are empty
                        >
                            提交评论
                        </button>
                        <button
                            className="bg-red-500 text-white px-4 py-2 rounded hover:bg-red-700"
                            onClick={() => {
                                setShowReplyForm(false);
                                setReplyToUser('');
                            }}
                        >
                            关闭
                        </button>
                    </div>
                </div>
            )}
            {!showReplyForm && (
                <div id="comment-form-container" className="comment-form mt-4 p-4 bg-gray-100 rounded-lg shadow">
                    <p className="mb-2">评论文章:</p>
                    <input
                        type="text"
                        id="comment-username"
                        placeholder="输入你的名字"
                        value={username}
                        onChange={(e) => setUsername(e.target.value)}
                        className="mb-2 p-2 border rounded w-full"
                    />
                    <textarea
                        id="comment-text"
                        placeholder="输入你的评论"
                        value={newComment}
                        onChange={(e) => setNewComment(e.target.value)}
                        required
                        className="mb-2 p-2 border rounded w-full"
                    />
                    <div className="flex justify-end">
                        <button
                            className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-700"
                            onClick={() => handleCommentSubmit()}
                            disabled={!username || !newComment} // Disable if fields are empty
                        >
                            提交评论
                        </button>
                    </div>
                </div>
            )}
        </div>
    );
};

export default CommentSection;
