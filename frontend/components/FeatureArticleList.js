// components/FeatureArticleList.js
import { useEffect, useState } from 'react';
import axios from 'axios';
import PostCard from './PostCard'; // Assuming PostCard is the component to display articles

const FeatureArticleList = ({ url }) => {
    const [featuredArticles, setFeaturedArticles] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    const fetchFeaturedArticles = async () => {
        try {
            setLoading(true);
            const response = await axios.get(url(), {
                headers: {
                    'Authorization': `Bearer ${sessionStorage.getItem('authToken')}`,
                },
                withCredentials: true
            });
            setFeaturedArticles(response.data);
        } catch (err) {
            setError(err);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchFeaturedArticles();
    }, [url]);

    if (loading) {
        return <div className="text-center">加载中...</div>;
    }

    if (error) {
        return <div className="text-center text-red-500">出错了: {error.message}</div>;
    }

    if (featuredArticles.length === 0) {
        return <div className="text-center text-gray-500">暂无文章显示</div>;
    }

    return (
        <div>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {featuredArticles.map((article) => (
                    <PostCard key={article.id} post={article} />
                ))}
            </div>
        </div>
    );
};

export default FeatureArticleList;