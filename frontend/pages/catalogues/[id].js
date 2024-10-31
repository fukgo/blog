import { useState, useEffect } from 'react';
import Link from 'next/link';
import axios from 'axios';
import { getCatalogueUrl, getCataloguesArticlesUrl } from '@/api_list';
import { useRouter } from 'next/router';

const CataloguesDetail = () => {
    const [catalogues, setCatalogues] = useState([]);

    const router = useRouter();

    const { id } = router.query;

    useEffect(() => {
        if (!id) return;

        const fetchCatalogues = async () => {
            try {
                const response = await axios.get(getCatalogueUrl(id));
                const catalogueData = response.data;

                if (catalogueData) {
                    const articlesResponse = await axios.get(getCataloguesArticlesUrl(catalogueData.id));
                    const articlesData = Array.isArray(articlesResponse.data) ? articlesResponse.data : [];
                    const catalogueWithArticles = { ...catalogueData, articles: articlesData, isOpen: false };
                    setCatalogues([catalogueWithArticles]);
                } else {
                    console.error('No catalogue data received');
                }
            } catch (error) {
                console.error('Error fetching catalogues:', error);
            }
        };

        fetchCatalogues();
    }, [id]);

    const toggleCollapse = (catalogueId) => {
        setCatalogues(prevCatalogues =>
            prevCatalogues.map(catalogue =>
                catalogue.id === catalogueId ? { ...catalogue, isOpen: !catalogue.isOpen } : catalogue
            )
        );
    };

    return (
        <div className="container mx-auto px-4 py-8">
            <h1 className="text-3xl font-bold mb-6 text-center">目录</h1>
            {catalogues.length === 0 ? (
                <p className="text-gray-500 text-center">No catalogues available.</p>
            ) : (
                catalogues.map(catalogue => (
                    <div key={catalogue.id} className="mb-6 bg-gray-100 rounded-lg shadow-md">
                        {/* 目录介绍 */}
                        <div className="bg-white p-4 rounded-t-lg">
                            <h2 className="text-2xl font-semibold text-blue-800">{catalogue.catalogue}</h2>
                            <p className="text-gray-600">{catalogue.info}</p>
                        </div>

                        {/* 折叠内容 */}
                        <div>
                            <div
                                className="cursor-pointer bg-blue-100 p-4 rounded flex justify-between items-center hover:bg-blue-200 transition-colors duration-300"
                                onClick={() => toggleCollapse(catalogue.id)}
                            >
                                <span className="font-semibold text-blue-800">文章列表</span>
                                <svg
                                    className={`w-6 h-6 text-blue-800 transform transition-transform ${catalogue.isOpen ? 'rotate-180' : ''}`}
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 9l-7 7-7-7"></path>
                                </svg>
                            </div>

                            {/* 折叠文章列表 */}
                            {catalogue.isOpen && (
                                <ul className="mt-4 bg-white p-6 rounded-b-lg">
                                    {catalogue.articles.map(article => (
                                        <li key={article.article_id} className="p-4 rounded-lg shadow-md border border-gray-100 hover:shadow-lg transition-shadow">
                                            <h3 className="text-lg font-semibold mb-1 text-blue-700">
                                                <Link href={`/posts/${article.article_id}`}>
                                                    {article.title}
                                                </Link>
                                            </h3>
                                            <p className="text-gray-600 text-sm">{article.digest}</p>
                                        </li>
                                    ))}
                                </ul>
                            )}
                        </div>
                    </div>
                ))
            )}
        </div>
    );
};

export default CataloguesDetail;
