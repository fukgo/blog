import Head from 'next/head';
import Link from 'next/link';
import { useEffect, useState } from 'react';
import axios from 'axios';

export default function Layout({ children }) {
    const [tags, setTags] = useState([]); // 存储标签的状态
    const [users, setUsers] = useState([]); // 存储用户的状态
    const [loadingTags, setLoadingTags] = useState(true); // 标签加载状态
    const [loadingUsers, setLoadingUsers] = useState(true); // 用户加载状态
    const [errorTags, setErrorTags] = useState(null); // 标签错误状态
    const [errorUsers, setErrorUsers] = useState(null); // 用户错误状态

    // 从 API 获取标签的函数
    const fetchTags = async () => {
        try {
            const token = sessionStorage.getItem('authToken'); // 从 sessionStorage 获取 token

            const response = await axios.get('http://127.0.0.1:8002/tags/all', {
                headers: {
                    'Authorization': `Bearer ${token}`, // 在请求头中添加 token
                },
                withCredentials: true
            });
            setTags(response.data); // 设置获取到的标签
        } catch (err) {
            setErrorTags(err); // 处理错误
        } finally {
            setLoadingTags(false); // 结束加载
        }
    };

    // 从 API 获取用户的函数
    const fetchUsers = async () => {
        try {
            const token = sessionStorage.getItem('authToken'); // 从 sessionStorage 获取 token

            const response = await axios.get('http://127.0.0.1:8002/users', {
                headers: {
                    'Authorization': `Bearer ${token}`, // 在请求头中添加 token
                },
                withCredentials: true
            });
            console.log(response.data);
            setUsers(response.data); // 设置获取到的用户
        } catch (err) {
            setErrorUsers(err); // 处理错误
        } finally {
            setLoadingUsers(false); // 结束加载
        }
    };

    // 组件挂载时获取标签和用户
    useEffect(() => {
        fetchTags();
        fetchUsers();
    }, []);

    // 处理加载状态
    if (loadingTags || loadingUsers) {
        return <div>加载中...</div>; // 显示加载状态
    }

    // 处理错误状态
    if (errorTags) {
        return <div>获取标签时出错: {errorTags.message}</div>; // 显示错误信息
    }

    if (errorUsers) {
        return <div>获取用户时出错: {errorUsers.message}</div>; // 显示错误信息
    }

    return (
        <div className="min-h-screen flex flex-col">
            <Head>
                <title>我的博客</title>
                <meta name="description" content="一个使用 Next.js 构建的个人博客" />
            </Head>

            {/* 头部 */}
            <header className="bg-white shadow-sm">
                <div className="container mx-auto px-4 py-4">
                    <nav className="flex justify-between items-center">
                        <Link href="/" className="text-2xl font-bold text-gray-900">我的博客</Link>
                        <div className="space-x-4">
                            <Link href="/about" className="text-gray-700 hover:text-gray-900">关于</Link>
                            <Link href="/" className="text-gray-700 hover:text-gray-900">博客</Link>
                            {/* <Link href="/" className="text-gray-700 hover:text-gray-900">联系</Link> */}

                            {/* 添加新文章/标签按钮 */}
                            <Link href="/posts/new" className="bg-blue-500 text-white px-3 py-1 rounded hover:bg-blue-600">新文章</Link>
                            <Link href="/tags/new" className="bg-green-500 text-white px-3 py-1 rounded hover:bg-green-600">新标签</Link>
                        </div>
                    </nav>
                </div>
            </header>

            {/* 主内容 */}
            <div className="flex flex-grow container mx-auto px-4 py-6">
                {/* 侧边栏 */}
                <aside className="w-64 bg-gray-50 border-r hidden lg:block">
                    <div className="p-6">
                        <h2 className="text-lg font-bold text-gray-900 mb-4">标签</h2>
                        <ul className="space-y-2">
                            {tags.map((tag) => (
                                <li key={tag.id}>
                                    <Link href={`/tags/${tag.id}`} className="text-gray-600 hover:text-gray-900">
                                        {tag.tag}
                                    </Link>
                                </li>
                            ))}
                        </ul>

                        {/* 作者部分 */}
                        <h2 className="text-lg font-bold text-gray-900 mt-6 mb-4">作者</h2>
                        <ul className="space-y-1">
                            {users.map((user) => (
                                <li key={user.id}>
                                    <Link href={`/users/${user.id}`} className="text-gray-600 hover:text-gray-900">
                                        {user.username}
                                    </Link>
                                </li>
                            ))}
                        </ul>

                    </div>
                </aside>

                {/* 主内容区域 */}
                <main className="flex-grow bg-white p-6 shadow-sm">
                    {children}
                </main>
            </div>

            {/* 页脚 */}
            <footer className="bg-white border-t py-4">
                <div className="container mx-auto text-center text-gray-500">
                    © 2024 the blog - All rights reserved
                </div>
            </footer>
        </div>
    );
}