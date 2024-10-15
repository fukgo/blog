// components/ArticleList.js
import PostCard from './PostCard';
import { useEffect, useState } from 'react';
import axios from 'axios';

const ArticleList = ({ url, page, limit, setPage, setLimit }) => {
    const [posts, setPosts] = useState([]); // 存储文章数据
    const [loading, setLoading] = useState(true); // 加载状态
    const [error, setError] = useState(null); // 错误状态
    const [totalPages, setTotalPages] = useState(1); // 总页数

    // 数据获取函数
    const fetchArticles = async () => {
        try {
            setLoading(true); // 开始加载

            // 从 sessionStorage 获取 token
            const token = sessionStorage.getItem('authToken');

            const response = await axios.get(url, {
                params: {
                    page: page,
                    limit: limit,
                },
                headers: {
                    // 在请求头中添加 token
                    'Authorization': `Bearer ${token}`,
                },
                withCredentials: true
            });

            setPosts(response.data); // 设置文章数据
            setTotalPages(response.data[0]?.total_page || 1); // 通过第一篇文章的 total_page 更新总页数
        } catch (err) {
            setError(err); // 处理错误
        } finally {
            setLoading(false); // 结束加载
        }
    };

    useEffect(() => {
        fetchArticles(); // 在组件挂载时获取文章
    }, [url, page, limit]); // 当 url, page 或 limit 变化时重新获取数据

    const handleNextPage = () => {
        if (page < totalPages) {
            setPage(page + 1); // 下一页
        }
    };

    const handlePrevPage = () => {
        if (page > 1) {
            setPage(page - 1); // 上一页
        }
    };

    if (loading) {
        return <div>Loading...</div>; // 显示加载状态
    }

    if (error) {
        return <div>Error: {error.message}</div>; // 显示错误信息
    }

    return (
        <div>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                {posts.map((post) => (
                    <PostCard key={post.id} post={post} />
                ))}
            </div>
            {/* 分页按钮 */}
            <div className="flex justify-between mt-4">
                <button onClick={handlePrevPage} disabled={page === 1} className="bg-gray-500 text-white px-4 py-2 rounded">
                    上一页
                </button>
                <button onClick={handleNextPage} disabled={page === totalPages} className="bg-gray-500 text-white px-4 py-2 rounded">
                    下一页
                </button>
            </div>
            <p className="mt-2">当前页: {page} / {totalPages}</p>
        </div>
    );
};

export default ArticleList;
