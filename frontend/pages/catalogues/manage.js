import { useState, useEffect } from 'react';
import Link from 'next/link';
import axios from 'axios';
import { getCataloguesAllUrl, getCataloguesArticlesUrl, deleteCatalogueArticleUrl, deleteCatalogueArticlesUrl, postCatalogueArticleUrl, postCatalogueArticleSortUrl, getArticlesTitlesUrl } from '@/api_list';
import ParentComponent from '@/components/ProtectedComponent';

const CataloguesDetail = () => {
    const [catalogues, setCatalogues] = useState([]);
    const [articles, setArticles] = useState([]);
    const [selectedArticleId, setSelectedArticleId] = useState('');
    const [sortOrder, setSortOrder] = useState({});
    const [newSortOrder, setNewSortOrder] = useState('');
    const [selectedCatalogueId, setSelectedCatalogueId] = useState(null);

    useEffect(() => {
        const fetchCatalogues = async () => {
            try {
                const response = await axios.get(getCataloguesAllUrl());
                const cataloguesData = Array.isArray(response.data) ? response.data : [];

                const fetchArticlesPromises = cataloguesData.map(catalogue =>
                    axios.get(getCataloguesArticlesUrl(catalogue.id))
                        .then(res => ({
                            ...catalogue,
                            articles: Array.isArray(res.data) ? res.data : [],
                            isOpen: false // 默认未展开
                        }))
                );

                const cataloguesWithArticles = await Promise.all(fetchArticlesPromises);
                setCatalogues(cataloguesWithArticles);
            } catch (error) {
                console.error('Error fetching catalogues:', error);
            }
        };

        const fetchArticles = async () => {
            try {
                const response = await axios.get(getArticlesTitlesUrl());
                setArticles(response.data);
            } catch (error) {
                console.error('Error fetching articles:', error);
            }
        };

        fetchCatalogues();
        fetchArticles();
    }, []);

    const toggleCollapse = (catalogueId) => {
        setCatalogues(prevCatalogues =>
            prevCatalogues.map(catalogue =>
                catalogue.id === catalogueId ? { ...catalogue, isOpen: !catalogue.isOpen } : catalogue
            )
        );
    };

    const handleAddArticle = async (catalogueId) => {
        try {
            console.log('selectedArticleId:', selectedArticleId);
            console.log('selectedCatalogueId:', catalogueId);
            console.log('newSortOrder:', newSortOrder);

            if (!catalogueId) {
                console.error('Catalogue ID is not set');
                return;
            }

            const response = await axios.post(postCatalogueArticleUrl(), {
                article_id: parseInt(selectedArticleId, 10),
                catalogue_id: parseInt(catalogueId, 10),
                sort_order: parseInt(newSortOrder, 10)
            });

            console.log('Response:', response);

            setSelectedArticleId('');
            setNewSortOrder('');
            setSelectedCatalogueId(null);

            // 刷新页面
            window.location.reload();
        } catch (error) {
            console.error('Error adding article:', error);
            if (error.response && error.response.data) {
                console.error('服务器返回的错误信息:', error.response.data);
            }
        }
    };

    const handleDeleteArticle = async (catalogueId, articleId) => {
        try {
            await axios.delete(deleteCatalogueArticleUrl(catalogueId, articleId));
            const response = await axios.get(getCataloguesAllUrl());
            const cataloguesData = Array.isArray(response.data) ? response.data : [];
            const fetchArticlesPromises = cataloguesData.map(catalogue =>
                axios.get(getCataloguesArticlesUrl(catalogue.id))
                    .then(res => ({
                        ...catalogue,
                        articles: Array.isArray(res.data) ? res.data : [],
                        isOpen: catalogue.isOpen // 保持展开状态
                    }))
            );
            const cataloguesWithArticles = await Promise.all(fetchArticlesPromises);
            setCatalogues(cataloguesWithArticles);
        } catch (error) {
            console.error('Error deleting article:', error);
        }
    };

    const handleDeleteAllArticles = async (catalogueId) => {
        try {
            await axios.delete(deleteCatalogueArticlesUrl(catalogueId));
            // Refresh catalogues
            const response = await axios.get(getCataloguesAllUrl());
            const cataloguesData = Array.isArray(response.data) ? response.data : [];
            const fetchArticlesPromises = cataloguesData.map(catalogue =>
                axios.get(getCataloguesArticlesUrl(catalogue.id))
                    .then(res => ({
                        ...catalogue,
                        articles: Array.isArray(res.data) ? res.data : [],
                        isOpen: catalogue.isOpen // 保持展开状态
                    }))
            );
            const cataloguesWithArticles = await Promise.all(fetchArticlesPromises);
            setCatalogues(cataloguesWithArticles);
        } catch (error) {
            console.error('Error deleting all articles:', error);
        }
    };

    const handleSortOrderChange = (articleId, catalogueId, newSortOrder) => {
        setSortOrder(prevSortOrder => ({
            ...prevSortOrder,
            [articleId]: { catalogueId, sortOrder: newSortOrder }
        }));
    };

    const handleEditArticle = async (articleId) => {
        window.location.href = `/posts/update/${articleId}`;
    };

    const handleSortOrderSubmit = async (articleId) => {
        const { catalogueId, sortOrder: newSortOrder } = sortOrder[articleId] || {};
        if (!catalogueId || !newSortOrder) {
            console.error('Invalid sort order data');
            return;
        }
        try {
            console.log('Updating sort order:', articleId, catalogueId, newSortOrder);
            await axios.post(postCatalogueArticleSortUrl(), {
                article_id: parseInt(articleId, 10),
                catalogue_id: parseInt(catalogueId, 10),
                sort_order: parseInt(newSortOrder, 10)
            });
            // Refresh catalogues
            const response = await axios.get(getCataloguesAllUrl());
            const cataloguesData = Array.isArray(response.data) ? response.data : [];
            const fetchArticlesPromises = cataloguesData.map(catalogue =>
                axios.get(getCataloguesArticlesUrl(catalogue.id))
                    .then(res => ({
                        ...catalogue,
                        articles: Array.isArray(res.data) ? res.data : [],
                        isOpen: catalogue.isOpen // 保持展开状态
                    }))
            );
            const cataloguesWithArticles = await Promise.all(fetchArticlesPromises);
            setCatalogues(cataloguesWithArticles);
        } catch (error) {
            console.error('Error updating sort order:', error);
        }
    };

    return (
        <ParentComponent>
            <div className="container mx-auto px-4 py-8">
                <h1 className="text-3xl font-bold mb-6">目录</h1>
                {catalogues.length === 0 ? (
                    <p className="text-gray-500">没有目录</p>
                ) : (
                    catalogues.map(catalogue => (
                        <div key={catalogue.id} className="mb-4"> {/* 减小目录间距 */}
                            <div
                                className={`bg-gray-200 p-4 rounded flex justify-between items-center cursor-pointer`}
                                onClick={() => toggleCollapse(catalogue.id)}
                            >
                                <div>
                                    <h2 className="text-2xl font-semibold">{catalogue.catalogue}</h2>
                                    {catalogue.info && <p className="text-gray-700">{catalogue.info}</p>}
                                    <p className="text-gray-600 text-sm">文章数量: {Array.isArray(catalogue.articles) ? catalogue.articles.length : 0}</p>
                                </div>
                                {Array.isArray(catalogue.articles) && catalogue.articles.length > 0 && (
                                    <svg
                                        className={`w-6 h-6 transform transition-transform ${catalogue.isOpen ? 'rotate-180' : ''}`}
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                        xmlns="http://www.w3.org/2000/svg"
                                    >
                                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 9l-7 7-7-7"></path>
                                    </svg>
                                )}
                            </div>
                            {catalogue.isOpen && (
                                <div>
                                    <ul className="mt-4 space-y-4">
                                        {Array.isArray(catalogue.articles) && catalogue.articles.map(article => (
                                            <li key={article.article_id} className="bg-white p-4 rounded shadow transition-transform hover:shadow-lg">
                                                <h3 className="text-xl font-bold mb-2">
                                                    <Link href={`/posts/${article.article_id}`}>
                                                        {article.title}
                                                    </Link>
                                                </h3>
                                                <p className="text-gray-500">次序: {article.sort_order}</p>
                                                <p className="text-gray-700">{article.digest}</p>
                                                <input
                                                    type="number"
                                                    step="1" // 限制只能输入整数
                                                    min="1" // 限制为正整数
                                                    value={sortOrder[article.article_id]?.sortOrder || article.sort_order}
                                                    onChange={(e) => {
                                                        const value = parseInt(e.target.value, 10);
                                                        handleSortOrderChange(article.article_id, catalogue.id, value > 0 ? value : '');
                                                    }}
                                                    placeholder="次序"
                                                    className="w-full p-2 mb-2 border rounded"
                                                />
                                                <button
                                                    className="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg mt-2 transition duration-200 ease-in-out"
                                                    onClick={() => handleSortOrderSubmit(article.article_id)}
                                                >
                                                    更新次序
                                                </button>

                                                <button
                                                    className="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg mt-2 ml-2 transition duration-200 ease-in-out"
                                                    onClick={() => handleDeleteArticle(catalogue.id, article.article_id)}
                                                >
                                                    删除文章
                                                </button>

                                                <button
                                                    className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg mt-2 ml-2 transition duration-200 ease-in-out"
                                                    onClick={() => handleEditArticle(article.article_id)}
                                                >
                                                    编辑文章
                                                </button>

                                            </li>
                                        ))}
                                    </ul>
                                    {Array.isArray(catalogue.articles) && catalogue.articles.length > 0 && (
                                        <button className="bg-red-500 text-white px-3 py-1 rounded mt-4" onClick={() => handleDeleteAllArticles(catalogue.id)}>删除所有文章</button>
                                    )}
                                    <div className="mt-4">
                                        <h3 className="text-xl font-bold mb-2">添加文章到目录</h3>
                                        <select
                                            value={selectedArticleId}
                                            onChange={(e) => setSelectedArticleId(e.target.value)}
                                            className="w-full p-2 mb-2 border rounded"
                                        >
                                            <option value="">选择文章</option>
                                            {articles.map(article => (
                                                <option key={article.id} value={article.id}>{article.title}</option>
                                            ))}
                                        </select>
                                        <input
                                            type="number"
                                            value={newSortOrder}
                                            onChange={(e) => setNewSortOrder(e.target.value)}
                                            placeholder="次序 (越小显示越前)"
                                            className="w-full p-2 mb-2 border rounded"
                                        />
                                        <button className="bg-blue-500 text-white px-4 py-2 rounded" onClick={() => handleAddArticle(catalogue.id)}>
                                            添加文章
                                        </button>
                                    </div>
                                </div>
                            )}
                        </div>
                    ))
                )}
            </div>
        </ParentComponent>
    );
};

export default CataloguesDetail;