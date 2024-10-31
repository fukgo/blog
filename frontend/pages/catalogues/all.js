import { useState, useEffect } from 'react';
import Link from 'next/link';
import axios from 'axios';
import { getCataloguesAllUrl, getCataloguesArticlesUrl } from '@/api_list';

const CataloguesDetail = () => {
    const [catalogues, setCatalogues] = useState([]);

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

        fetchCatalogues();
    }, []);

    const toggleCollapse = (catalogueId) => {
        setCatalogues(prevCatalogues =>
            prevCatalogues.map(catalogue =>
                catalogue.id === catalogueId ? { ...catalogue, isOpen: !catalogue.isOpen } : catalogue
            )
        );
    };

    return (
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
                                <p className="text-gray-600 text-sm">Articles: {catalogue.articles.length}</p>
                            </div>
                            {catalogue.articles.length > 0 && (
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
                            <ul className="mt-4 space-y-4">
                                {catalogue.articles.map(article => (
                                    <li key={article.article_id} className="bg-white p-4 rounded shadow transition-transform hover:shadow-lg">
                                        <h3 className="text-xl font-bold mb-2">
                                            <Link href={`/posts/${article.article_id}`}>
                                                {article.title}
                                            </Link>
                                        </h3>
                                        <p className="text-gray-700">{article.digest}</p>
                                    </li>
                                ))}
                            </ul>
                        )}
                    </div>
                ))
            )}
        </div>
    );
};

export default CataloguesDetail;
