import React, { useState, useEffect, useRef } from 'react';
import axios from 'axios';
import dynamic from 'next/dynamic';
import 'react-quill/dist/quill.snow.css';
import ProtectedComponent from "@/components/ProtectedComponent";

const ReactQuill = dynamic(() => import('react-quill'), { ssr: false });

const NewPost = () => {
    const [title, setTitle] = useState('');
    const [digest, setDigest] = useState('');
    const [content, setContent] = useState('');
    const [tags, setTags] = useState([]);
    const [selectedTags, setSelectedTags] = useState([]);
    const [userId, setUserId] = useState(null);
    const [newTag, setNewTag] = useState('');
    const quillRef = useRef();

    useEffect(() => {
        const fetchUserAndTags = async () => {
            try {
                const userResponse = await axios.get('http://127.0.0.1:8002/auth/session', { withCredentials: true });
                setUserId(userResponse.data.id);

                const tagsResponse = await axios.get('http://127.0.0.1:8002/tags/all', { withCredentials: true });
                setTags(tagsResponse.data);
            } catch (error) {
                console.error('获取用户或标签时出错:', error);
            }
        };

        fetchUserAndTags();
    }, []);

    const handleTagChange = (tagId) => {
        setSelectedTags((prevSelectedTags) =>
            prevSelectedTags.includes(tagId)
                ? prevSelectedTags.filter((id) => id !== tagId)
                : [...prevSelectedTags, tagId]
        );
    };

    const handleAddTag = () => {
        if (newTag.trim() !== '') {
            setTags([...tags, { id: Date.now(), tag: newTag.trim() }]);
            setNewTag('');
        }
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        try {
            const newPost = {
                title,
                content,
                digest,
                user_id: userId,
                tags_id: selectedTags,
            };
            await axios.post('http://127.0.0.1:8002/articles', newPost, { withCredentials: true });
            alert('文章创建成功！');
        } catch (error) {
            console.error('创建文章时出错:', error);
        }
    };

    return (
        <ProtectedComponent>
            <div className="container mx-auto p-6 max-w-4xl">
                <h1 className="text-3xl font-bold mb-6">创建新文章</h1>
                <form onSubmit={handleSubmit}>
                    <div className="mb-4">
                        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="title">
                            标题
                        </label>
                        <input
                            id="title"
                            type="text"
                            value={title}
                            onChange={(e) => setTitle(e.target.value)}
                            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            required
                        />
                    </div>
                    <div className="mb-4">
                        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="digest">
                            摘要
                        </label>
                        <input
                            id="digest"
                            type="text"
                            value={digest}
                            onChange={(e) => setDigest(e.target.value)}
                            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            required
                        />
                    </div>
                    <div className="mb-4">
                        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="content">
                            内容
                        </label>
                        <ReactQuill
                            ref={quillRef}
                            value={content}
                            onChange={setContent}
                            style={{ height: '400px' }}
                        />
                    </div>
                    <div className="mb-4">
                        <label className="block text-gray-700 text-sm font-bold mb-4">标签</label>
                        <div className="flex flex-wrap mb-2">
                            {selectedTags.map((tagId) => {
                                const tag = tags.find(t => t.id === tagId);
                                return (
                                    tag && (
                                        <span key={tag.id}
                                              className="inline-block bg-blue-200 rounded-full px-3 py-1 text-sm font-semibold text-blue-700 mr-2 mb-2">
                                            {tag.tag}
                                        </span>
                                    )
                                );
                            })}
                        </div>
                        {tags.map((tag) => (
                            <div key={tag.id} className="inline-block mr-4">
                                <input
                                    type="checkbox"
                                    id={`tag-${tag.id}`}
                                    value={tag.id}
                                    onChange={() => handleTagChange(tag.id)}
                                    className="mr-2 leading-tight"
                                />
                                <label htmlFor={`tag-${tag.id}`} className="text-gray-700">
                                    {tag.tag}
                                </label>
                            </div>
                        ))}
                        <div className="flex mt-4">
                            <input
                                type="text"
                                value={newTag}
                                onChange={(e) => setNewTag(e.target.value)}
                                placeholder="添加新标签"
                                className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            />
                            <button
                                type="button"
                                onClick={handleAddTag}
                                className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded ml-2"
                            >
                                添加
                            </button>
                        </div>
                    </div>
                    <button
                        type="submit"
                        className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    >
                        创建文章
                    </button>
                </form>
            </div>
        </ProtectedComponent>
    );
};

export default NewPost;