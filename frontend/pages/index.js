// pages/index.js
import Head from 'next/head';
import { useState, useEffect } from 'react';
import ArticleList from '../components/ArticleList';
import FeatureArticleList from '../components/FeatureArticleList';
import { authTokenUrl, getLateArticlesUrl, getFeatureArticleUrl } from "@/api_list";
import { loginUrl } from "@/api_list";

export default function Home() {
    const [page, setPage] = useState(1);
    const [limit, setLimit] = useState(10);
    const [featuredPage, setFeaturedPage] = useState(1);
    const [featuredLimit, setFeaturedLimit] = useState(10);
    const [modalMessage, setModalMessage] = useState(null);
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [isLoginSuccessful, setIsLoginSuccessful] = useState(false);

    useEffect(() => {
        const urlParams = new URLSearchParams(window.location.search);
        const token = urlParams.get('token');
        if (token) {
            fetch(authTokenUrl(), {
                method: 'GET',
                headers: {
                    'Authorization': `Bearer ${token}`
                },
                credentials: 'include'
            })
                .then(response => {
                    if (response.status === 200) {
                        return response.json();
                    } else {
                        setModalMessage('登录失败，请检查您的凭据。');
                        setIsModalOpen(true);
                        throw new Error('Login failed');
                    }
                })
                .then(user => {
                    const item = {
                        user: user,
                        expiry: Date.now() + 5400 * 1000 // 1.5 hours
                    };
                    sessionStorage.setItem('user', JSON.stringify(item));
                    setModalMessage('登录成功');
                    setIsLoginSuccessful(true);
                    setIsModalOpen(true);

                    // Refresh page after showing the modal for 3 seconds
                    setTimeout(() => {
                        window.location.href = '/';
                    }, 3000);
                })
                .catch(error => {
                    setModalMessage('登录失败，请稍后重试。');
                    setIsModalOpen(true);
                    setIsLoginSuccessful(false);
                });
        }
    }, []);

    const handleModalClose = () => {
        setIsModalOpen(false);
        if (!isLoginSuccessful) {
            window.location.href = '/'; // Redirect to home if login failed
        }
    };

    return (
        <div>
            <Head>
                <title>博客平台</title>
                <meta name="description" content="Welcome to my blog where I share knowledge about web development, programming, and more." />
            </Head>

            {/* Header section */}
            <section className="bg-gradient-to-r from-blue-500 to-green-500 py-20 text-center text-white">
                <div className="container mx-auto">
                    <h1 className="text-5xl font-extrabold mb-4">欢迎来到博客平台</h1>
                    <p className="text-lg">分享来自计算机方向的任何知识</p>
                </div>
            </section>

            {/* Featured articles section */}
            <section className="py-12 bg-gray-50">
                <div className="container mx-auto px-4">
                    <h2 className="text-3xl font-bold mb-6 text-center">精选文章</h2>
                    <FeatureArticleList
                        url={getFeatureArticleUrl}
                        page={featuredPage}
                        limit={featuredLimit}
                        setPage={setFeaturedPage}
                        setLimit={setFeaturedLimit}
                    />
                </div>
            </section>

            {/* Latest articles section */}
            <section className="py-12 bg-gray-50">
                <div className="container mx-auto px-4">
                    <h2 className="text-3xl font-bold mb-6 text-center">最新文章</h2>
                    <ArticleList
                        url={getLateArticlesUrl}
                        page={page}
                        limit={limit}
                        setPage={setPage}
                        setLimit={setLimit}
                    />
                </div>
            </section>

            {/* Custom Modal */}
            {isModalOpen && (
                <div className="fixed inset-0 flex items-center justify-center z-50">
                    <div className="bg-white p-6 rounded shadow-lg max-w-sm w-full">
                        <h2 className="text-xl font-bold mb-4">{isLoginSuccessful ? '登录成功' : '登录失败'}</h2>
                        <p className="mb-4">{modalMessage}</p>
                        <div className="flex justify-end">
                            {!isLoginSuccessful && (
                                <>
                                    <button
                                        className="bg-blue-500 text-white px-4 py-2 rounded mr-2"
                                        onClick={handleModalClose}
                                    >
                                        继续
                                    </button>
                                    <button
                                        className="bg-gray-500 text-white px-4 py-2 rounded"
                                        onClick={() => (window.location.href = '/')}
                                    >
                                        返回
                                    </button>
                                </>
                            )}
                        </div>
                    </div>
                    {!isLoginSuccessful && <div className="fixed inset-0 bg-black opacity-50"></div>}
                </div>
            )}
        </div>
    );
}