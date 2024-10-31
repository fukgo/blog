import React, { useState, useEffect, useRef } from 'react';
import axios from 'axios';
import dynamic from 'next/dynamic';
import ReactMarkdown from 'react-markdown';
import 'react-quill/dist/quill.snow.css';
import ProtectedComponent from "@/components/ProtectedComponent";
import { getArticleDetailUrl, updateArticleUrl, getAllTagsUrl, getArticleTagsUrl } from "@/api_list";
import remarkGfm from 'remark-gfm';
import useAuth from '@/components/useAuth';
import Modal from '@/components/Modal'; // 引入 Modal 组件
import { useRouter } from 'next/router';


const EditPost = () => {
    const router = useRouter();
    const { id } = router.query;
    const [ isAuthenticated, loading ] = useAuth();
    const [title, setTitle] = useState('');
    const [digest, setDigest] = useState('');
    const [content, setContent] = useState('');
    const [tags, setTags] = useState([]);
    const [selectedTags, setSelectedTags] = useState([]);
    const [isFeatured, setIsFeatured] = useState(false);
    const [isModalOpen, setIsModalOpen] = useState(false); // 控制模态框的状态
    const [userDetailId, setUserDetailId] = useState(null); // 用户ID
    const [user, setUser] = useState(null);
    const quillRef = useRef();

    const getUser = async () => {
        const user = JSON.parse(sessionStorage.getItem('user'));
        if (user && user.user) {
            setUser(user.user);
            setUserDetailId(user.user.user_detail_id); // 在这里设置 userDetailId
        } else {
            console.error('用户信息未找到');
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
        if (isAuthenticated) {
            getUser();
        } else if (!loading) {
            setIsModalOpen(true); // 用户未登录，打开模态框
        }
    }, [isAuthenticated, loading]);

    useEffect(() => {
        if (id) {
            const fetchArticleDetail = async () => {
                try {
                    const response = await axios.get(getArticleDetailUrl(id), { withCredentials: true });
                    const article = response.data;
                    setTitle(article.title);
                    setDigest(article.digest);
                    setContent(article.content);
                    setIsFeatured(article.feature);

                    // 获取文章标签
                    const tagsResponse = await axios.get(getArticleTagsUrl(id), { withCredentials: true });
                    const articleTags = tagsResponse.data.map(tag => tag.id);
                    setSelectedTags(articleTags);
                } catch (error) {
                    console.error('获取文章详情时出错:', error);
                }
            };

            fetchArticleDetail();
        }
    }, [id]);

    const handleTagChange = (tagId) => {
        setSelectedTags((prevSelectedTags) =>
            prevSelectedTags.includes(tagId)
                ? prevSelectedTags.filter((id) => id !== tagId)
                : [...prevSelectedTags, tagId]
        );
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        if (!userDetailId) {
            setIsModalOpen(true); // 确保在提交之前用户已登录
            return;
        }
    
        console.log('Selected Tags:', selectedTags); // 检查选中的标签
        console.log('Is Featured:', isFeatured);     // 检查是否为精选
    
        try {
            const updatedPost = {
                title,
                content,
                digest,
                user_detail_id: userDetailId,
                feature: isFeatured,
                tags_id: selectedTags.map(tagId => parseInt(tagId, 10)), // 确保 tags_id 是整数数组
            };
            await axios.post(updateArticleUrl(id), updatedPost, { withCredentials: true });
            alert('文章更新成功！');
            router.push(`/posts/${id}`);
        } catch (error) {
            console.error('更新文章时出错:', error);
        }
    };
    

    return (
        <ProtectedComponent>
            <Modal
                isOpen={isModalOpen}
                onClose={() => setIsModalOpen(false)}
                onLogin={() => window.location.href = '/login'} // 导航到登录页面
            />
            <div className="container mx-auto p-6 max-w-4xl bg-white shadow-lg rounded-lg">
                <h1 className="text-3xl font-bold mb-6 text-center text-blue-600">编辑文章</h1>
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
                                        checked={selectedTags.includes(tag.id)}
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
                            更新文章
                        </button>
                    </div>
                </form>
            </div>
        </ProtectedComponent>
    );
};

export default EditPost;