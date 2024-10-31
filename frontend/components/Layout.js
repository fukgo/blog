import Head from 'next/head';
import Link from 'next/link';
import { useEffect, useState } from 'react';
import axios from 'axios';
import { getAllTagsUrl, getAllUsersUrl, logoutUrl, getCataloguesAllUrl } from '@/api_list';
import useAuth from "@/components/useAuth";
import { loginUrl } from "@/api_list";
import Image from 'next/image';

const logout = async () => {
    try {
        // Clear session storage
        sessionStorage.removeItem('user');

        // Call the logout API
        const response = await axios.delete(logoutUrl(), { withCredentials: true });
        console.log('Logout response:', response);

        // Redirect to login page
        window.location.href = "/";
    } catch (error) {
        console.error('Logout failed:', error);
    }
};

export default function Layout({ children }) {
    const [tags, setTags] = useState([]);
    const [users, setUsers] = useState([]);
    const [loadingTags, setLoadingTags] = useState(true);
    const [loadingUsers, setLoadingUsers] = useState(true);
    const [errorTags, setErrorTags] = useState(null);
    const [errorUsers, setErrorUsers] = useState(null);
    const [openDropdown, setOpenDropdown] = useState(null);
    const [catalogues, setCatalogues] = useState([]);
    const [loadingCatalogues, setLoadingCatalogues] = useState(true);
    const [tagsOpen, setTagsOpen] = useState(false);
    const [usersOpen, setUsersOpen] = useState(false);
    const [user, setUser] = useState({});
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [loading, setLoading] = useState(true);

    const catalogueToggleDropdown = () => {
        setOpenDropdown(openDropdown === 'catalogue' ? null : 'catalogue');
    };

    const userToggleDropdown = () => {
        setOpenDropdown(openDropdown === 'user' ? null : 'user');
    };

    const checkAuth = async () => {
        try {
            // 从 sessionStorage 获取用户信息
            const userData = JSON.parse(sessionStorage.getItem('user'));
            console.log('userData:', userData);
            // 检查用户数据是否存在
            if (userData) {
                // 检查用户数据是否过期
                if (userData.expiry > Date.now()) {
                    setIsAuthenticated(true); // 用户已登录
                    setUser(userData.user); // 更新用户数据
                } else {
                    console.warn('用户数据已过期');
                    setIsAuthenticated(false); // 用户未登录
                }
            } else {
                console.warn('用户数据不存在');
                setIsAuthenticated(false); // 用户未登录
            }
        } catch (error) {
            console.error('检查认证时发生错误:', error);
            setIsAuthenticated(false); // 请求失败，视为未登录
        } finally {
            setLoading(false); // 请求结束，更新加载状态
        }
    };

    const fetchTags = async () => {
        try {
            const token = sessionStorage.getItem('authToken');
            const response = await axios.get(getAllTagsUrl(), {
                headers: {
                    'Authorization': `Bearer ${token}`,
                },
                withCredentials: true
            });
            setTags(response.data);
        } catch (err) {
            setErrorTags(err);
        } finally {
            setLoadingTags(false);
        }
    };

    const fetchCatalogues = async () => {
        try {
            const token = sessionStorage.getItem('authToken');
            const response = await axios.get(getCataloguesAllUrl(), {
                headers: {
                    'Authorization': `Bearer ${token}`,
                },
                withCredentials: true
            });
            setCatalogues(response.data);
        } catch (err) {
            setErrorTags(err);
        } finally {
            setLoadingCatalogues(false);
        }
    };

    const fetchUsers = async () => {
        try {
            const token = sessionStorage.getItem('authToken');
            const response = await axios.get(getAllUsersUrl(), {
                headers: {
                    'Authorization': `Bearer ${token}`,
                },
                withCredentials: true
            });
            setUsers(response.data);
        } catch (err) {
            setErrorUsers(err);
        } finally {
            setLoadingUsers(false);
        }
    };

    useEffect(() => {
        checkAuth();
        fetchTags();
        fetchUsers();
        fetchCatalogues();
    }, []);

    useEffect(() => {
        console.log('isAuthenticated:', isAuthenticated); // 输出用户登录状态
        console.log('user:', user); // 输出用户数据
    }, [isAuthenticated, user]);

    return (
        <div className="min-h-screen flex flex-col">
            <Head>
                <title>我的博客</title>
                <meta name="description" content="这是一个使用 Next.js 构建的个人博客"/>
            </Head>

            <header className="bg-white shadow-sm">
                <div className="container mx-auto px-4 py-4">
                    <nav className="flex justify-between items-center">
                        <Link href="/" className="text-2xl font-bold text-gray-900">我的博客</Link>
                        <div className="space-x-4">
                            <Link href="/about" className="text-gray-700 hover:text-gray-900">关于</Link>
                            <Link href="/" className="text-gray-700 hover:text-gray-900">博客</Link>
                            <div className="relative inline-block">
                                <button
                                    onClick={catalogueToggleDropdown}
                                    className="flex items-center bg-gray-200 px-3 py-2 rounded-md transition duration-200 ease-in-out hover:bg-gray-300 focus:outline-none"
                                >
                                    <span className="text-gray-700 font-semibold">目录</span>
                                    <svg className="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                                         xmlns="http://www.w3.org/2000/svg">
                                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"
                                              d="M19 9l-7 7-7-7"></path>
                                    </svg>
                                </button>
                                {openDropdown === 'catalogue' && (
                                    <div className="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg z-10">
                                        {loadingCatalogues ? (
                                            <div className="space-y-2 p-2">
                                                {[...Array(5)].map((_, i) => (
                                                    <div key={i}
                                                         className="h-4 bg-gray-300 rounded animate-pulse"></div>
                                                ))}
                                            </div>
                                        ) : errorTags ? (
                                            <div className="text-red-600 p-2">获取目录时出错: {errorTags.message}</div>
                                        ) : (
                                            <ul className="space-y-2 p-2">
                                                {catalogues.map(catalogue => (
                                                    <li key={catalogue.id}>
                                                        <Link href={`/catalogues/${catalogue.id}`}
                                                              className="block text-gray-600 hover:text-gray-900 p-2 rounded transition duration-150 ease-in-out hover:bg-gray-100">
                                                            {catalogue.catalogue}
                                                        </Link>
                                                    </li>
                                                ))}
                                            </ul>
                                        )}
                                    </div>
                                )}
                            </div>

                            {loading ? (
                                <div className="flex items-center justify-center">
                                    <span>Loading...</span>
                                </div>
                            ) : isAuthenticated ? (
                                <div className="relative inline-block">
                                    <button onClick={userToggleDropdown}
                                            className="flex items-center bg-gray-200 px-3 py-1 rounded">
                                        <img src={user.avatar || 'err'} alt="avatar"
                                             className="w-8 h-8 rounded-full mr-2"/>
                                        <span className="text-gray-700">{user.nickname || "err name"}</span>
                                    </button>
                                    {openDropdown === 'user' && (
                                        <div className="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg z-10">
                                            <Link href={"/users/" + user.user_detail_id}
                                                  className="block px-4 py-2 text-gray-800 hover:bg-gray-100">主页</Link>
                                            <Link href="/posts/new"
                                                  className="block px-4 py-2 text-gray-800 hover:bg-gray-100">新文章</Link>
                                            <Link href="/tags/new"
                                                  className="block px-4 py-2 text-gray-800 hover:bg-gray-100">新标签</Link>
                                            <Link href="/catalogues/new"
                                                  className="block px-4 py-2 text-gray-800 hover:bg-gray-100">新目录</Link>
                                            <Link href="/catalogues/manage"
                                                  className="block px-4 py-2 text-gray-800 hover:bg-gray-100">目录管理</Link>
                                            <button onClick={logout}
                                                    className="block w-full text-left px-4 py-2 text-red-600 hover:bg-red-100">退出登录
                                            </button>
                                        </div>
                                    )}
                                </div>
                            ) : (
                                <Link href={loginUrl()}
                                      className="bg-blue-500 text-white px-3 py-1 rounded hover:bg-blue-600">登录</Link>
                            )}
                        </div>
                    </nav>
                </div>
            </header>

            <div className="flex flex-grow container mx-auto px-4 py-6">
                <aside className="w-64 bg-gray-50 border-r hidden lg:block">
                    <div className="p-6">
                        <Link href="/catalogues/all">
                            <h2 className="text-xl font-extrabold mb-4 cursor-pointer text-gray-800
               transition-all duration-300 transform hover:scale-105
               hover:text-teal-600 active:scale-95 shadow-lg hover:shadow-xl
               rounded-lg px-2 py-1 bg-gradient-to-r from-blue-200 via-teal-200 to-green-200 text-gray-900">
                                目录大全
                            </h2>
                        </Link>

                        <div>
                            <h2
                                className="text-lg font-bold text-gray-900 mb-4 cursor-pointer flex justify-between items-center"
                                onClick={() => setTagsOpen(!tagsOpen)}
                            >
                                标签
                                <span
                                    className={`transform transition-transform ${tagsOpen ? 'rotate-180' : 'rotate-0'}`}>▼</span>
                            </h2>
                            {tagsOpen && (
                                loadingTags ? (
                                    <div className="space-y-2">
                                        {[...Array(5)].map((_, i) => (
                                            <div key={i} className="h-4 bg-gray-300 rounded animate-pulse"></div>
                                        ))}
                                    </div>
                                ) : errorTags ? (
                                    <div className="text-red-600">获取标签时出错: {errorTags.message}</div>
                                ) : (
                                    <ul className="space-y-2">
                                        {tags.map((tag) => (
                                            <li key={tag.id}>
                                                <Link
                                                    href={`/tags/${tag.id}`}
                                                    className="text-gray-600 hover:text-gray-900 transition-all duration-200
                                               transform hover:scale-105 active:scale-95"
                                                >
                                                    {tag.tag}
                                                </Link>
                                            </li>
                                        ))}
                                    </ul>
                                )
                            )}
                        </div>

                        <div className="mt-6">
                            <h2
                                className="text-lg font-bold text-gray-900 mb-4 cursor-pointer flex justify-between items-center"
                                onClick={() => setUsersOpen(!usersOpen)}
                            >
                                作者
                                <span
                                    className={`transform transition-transform ${usersOpen ? 'rotate-180' : 'rotate-0'}`}>▼</span>
                            </h2>
                            {usersOpen && (
                                loadingUsers ? (
                                    <div className="space-y-2">
                                        {[...Array(3)].map((_, i) => (
                                            <div key={i} className="h-4 bg-gray-300 rounded animate-pulse"></div>
                                        ))}
                                    </div>
                                ) : errorUsers ? (
                                    <div className="text-red-600">获取用户时出错: {errorUsers.message}</div>
                                ) : (
                                    <ul className="space-y-1">
                                        {users.map((u) => (
                                            <li key={u.id} className="flex items-center">
                                                <img
                                                    src={`${u.avatar}`}
                                                    className="w-8 h-8 rounded-full mr-2"
                                                />
                                                <Link
                                                    href={`/users/${u.id}`}
                                                    className="text-gray-600 hover:text-gray-900 transition-all duration-200
                                               transform hover:scale-105 active:scale-95 flex items-center"
                                                >
                                                    {u.nickname}
                                                </Link>
                                            </li>
                                        ))}
                                    </ul>
                                )
                            )}
                        </div>
                    </div>
                </aside>

                <main className="flex-grow bg-white p-6 shadow-sm">
                    {children}
                </main>
            </div>

            <footer className="bg-white border-t py-8">
                <div className="container mx-auto text-center text-gray-600">
                    <div className="flex flex-col md:flex-row justify-between items-center">
                        <div className="mb-4 md:mb-0">
                            © 2024 个人博客 - 保留所有权利
                        </div>
                        <div className="flex space-x-4 mb-4 md:mb-0">
                            <Link href="/about" className="hover:underline">关于我们</Link>
                            {/*<Link href="/contact" className="hover:underline">联系我们</Link>*/}
                            {/*<Link href="/privacy" className="hover:underline">隐私政策</Link>*/}
                            {/*<Link href="/terms" className="hover:underline">服务条款</Link>*/}
                        </div>
                        <div className="flex space-x-4 mb-4 md:mb-0">
                            <Link href="https://github.com/fukgo/blog"
                                  className="text-gray-500 hover:text-gray-800">Github</Link>
                        </div>
                    </div>
                    <div className="mt-4">
                        <button className="text-sm text-gray-400 hover:text-gray-600"
                                onClick={() => window.scrollTo({top: 0, behavior: 'smooth'})}>
                            回到顶部
                        </button>
                    </div>
                </div>
            </footer>
        </div>
    );
}