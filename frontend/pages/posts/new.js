import React, { useState, useEffect } from 'react';
import axios from 'axios';
import ReactMarkdown from 'react-markdown';
import ProtectedComponent from "@/components/ProtectedComponent";
import { createArticleUrl, getAllTagsUrl } from "@/api_list";
import remarkGfm from 'remark-gfm';
import useAuth from '@/components/useAuth';

const NewPost = () => {
    const { isAuthenticated, loading } = useAuth();
    const [title, setTitle] = useState('');
    const [digest, setDigest] = useState('');
    const [content, setContent] = useState('');
    const [tags, setTags] = useState([]);
    const [selectedTags, setSelectedTags] = useState([]);
    const [isFeatured, setIsFeatured] = useState(false);
    const [userDetailId, setUserDetailId] = useState(null); // 用户ID
    const [user, setUser] = useState(null);
    console.log(isFeatured);

    const getUser = () => {
        try {
            const user = JSON.parse(sessionStorage.getItem('user'));
            if (user && user.user) {
                setUser(user.user);
                setUserDetailId(user.user.user_detail_id);
                console.log('User:', user.user);
                console.log('User Detail ID:', user.user.user_detail_id);
            } else {
                console.error('用户信息未找到');
            }
        } catch (error) {
            console.error('解析用户信息时出错:', error);
        }
    };
    

    useEffect(() => {
        const fetchTags = async () => {
            try {
                const tagsResponse = await axios.get(getAllTagsUrl(), { withCredentials: true });
                setTags(tagsResponse.data);
            } catch (error) {
                console.error('获取标签时出错:', error);
            }
        };

        fetchTags();
    }, []);

    useEffect(() => {
            getUser();
    }, []);

    const handleTagChange = (tagId) => {
        setSelectedTags((prevSelectedTags) =>
            prevSelectedTags.includes(tagId)
                ? prevSelectedTags.filter((id) => id !== tagId)
                : [...prevSelectedTags, tagId]
        );
    };

    const handleSubmit = async (e) => {
        console.log(user);
        console.log(userDetailId);
        e.preventDefault();
        console.log('handleSubmit called');
        console.log('User:', user);
        console.log('User Detail ID:', userDetailId);
        if (!user || !user.user_detail_id) {
            console.error('User Detail ID is not set');
            return;
        }

        try {
            const newPost = {
                title,
                content,
                digest,
                user_detail_id: userDetailId,
                tags_id: selectedTags,
                feature: isFeatured,
            };
            console.log('Submitting new post:', newPost);
            const response = await axios.post(createArticleUrl(), newPost, { withCredentials: true });
            console.log('Response:', response);
            alert('文章创建成功！');
        } catch (error) {
            console.error('创建文章时出错:', error);
        }
    };

    return (
        <ProtectedComponent>
            <div className="container mx-auto p-6 max-w-4xl bg-white shadow-lg rounded-lg">
                <h1 className="text-3xl font-bold mb-6 text-center text-blue-600">创建新文章</h1>
                <form onSubmit={handleSubmit} className="space-y-6">
                    {/* Title Input */}
                    <div>
                        <label htmlFor="title" className="block text-gray-700 font-semibold mb-2">标题</label>
                        <input
                            id="title"
                            type="text"
                            value={title}
                            onChange={(e) => setTitle(e.target.value)}
                            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 focus:outline-none focus:shadow-outline"
                            required
                        />
                    </div>

                    {/* Digest Input */}
                    <div>
                        <label htmlFor="digest" className="block text-gray-700 font-semibold mb-2">摘要</label>
                        <input
                            id="digest"
                            type="text"
                            value={digest}
                            onChange={(e) => setDigest(e.target.value)}
                            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 focus:outline-none focus:shadow-outline"
                            required
                        />
                    </div>

                    {/* Markdown Content Input */}
                    <div>
                        <label htmlFor="content" className="block text-gray-700 font-semibold mb-2">Markdown 内容</label>
                        <textarea
                            id="content"
                            value={content}
                            onChange={(e) => setContent(e.target.value)}
                            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 focus:outline-none focus:shadow-outline h-48 resize-none"
                            placeholder="在这里输入Markdown格式的内容..."
                            required
                        />
                    </div>

                    {/* Markdown Preview */}
                    <div className="mt-6">
                        <h2 className="text-lg font-semibold text-gray-700 mb-2">Markdown 预览</h2>
                        <div className="markdown-body border rounded p-4 bg-gray-50 text-gray-800">
                            <ReactMarkdown remarkPlugins={[remarkGfm]}>{content}</ReactMarkdown>
                        </div>
                    </div>

                    {/* Tags Selection */}
                    <div className="space-y-2">
                        <label className="block text-gray-700 font-semibold">标签</label>
                        <div className="flex flex-wrap">
                            {tags.map((tag) => (
                                <label key={tag.id} className="inline-flex items-center mr-4 mb-2">
                                    <input
                                        type="checkbox"
                                        value={tag.id}
                                        onChange={() => handleTagChange(tag.id)}
                                        className="mr-2"
                                    />
                                    <span className="text-gray-700">{tag.tag}</span>
                                </label>
                            ))}
                        </div>
                    </div>

                    {/* Featured Article Option */}
                    <div className="space-y-2">
                        <label className="block text-gray-700 font-semibold">设为精选文章</label>
                        <input
                            type="checkbox"
                            checked={isFeatured}
                            onChange={() => setIsFeatured(!isFeatured)}
                            className="mr-2"
                        />
                    </div>

                    {/* Submit Button */}
                    <div className="text-center">
                        <button
                            type="submit"
                            className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded focus:outline-none focus:shadow-outline"
                        >
                            创建文章
                        </button>
                    </div>
                </form>
            </div>
        </ProtectedComponent>
    );
};

export default NewPost;