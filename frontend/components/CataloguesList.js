import { useState, useEffect } from 'react';
import axios from 'axios';
import { getCataloguesAllUrl, deleteCatalogueUrl, postUpdateCatalogueUrl } from '@/api_list';

const CataloguesList = () => {
    const [catalogues, setCatalogues] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {
        fetchCatalogues();
    }, []);

    const fetchCatalogues = async () => {
        try {
            const response = await axios.get(getCataloguesAllUrl());
            setCatalogues(response.data);
        } catch (err) {
            setError(err);
        } finally {
            setLoading(false);
        }
    };

    const deleteCatalogue = async (catalogueId) => {
        try {
            await axios.delete(deleteCatalogueUrl(catalogueId));
            setCatalogues(catalogues.filter(catalogue => catalogue.id !== catalogueId));
        } catch (err) {
            setError(err);
        }
    };

    const updateCatalogue = async (catalogueId, newName) => {
        try {
            await axios.post(postUpdateCatalogueUrl(catalogueId), { name: newName });
            fetchCatalogues();
        } catch (err) {
            setError(err);
        }
    };

    if (loading) return <div>Loading...</div>;
    if (error) return <div>Error: {error.message}</div>;

    return (
        <div>
            <h2>目录列表</h2>
            <ul>
                {catalogues.map(catalogue => (
                    <li key={catalogue.id}>
                        {catalogue.name}
                        <button onClick={() => deleteCatalogue(catalogue.id)}>删除</button>
                        <button onClick={() => updateCatalogue(catalogue.id, prompt('新目录名:'))}>更新</button>
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default CataloguesList;