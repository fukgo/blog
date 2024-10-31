import { useState, useEffect } from 'react';
import axios from 'axios';
import { postCatalogueUrl, getCataloguesAllUrl, deleteCatalogueUrl, postUpdateCatalogueUrl } from '@/api_list';
import ParentComponent from '@/components/ProtectedComponent';

const CataloguesNew = () => {
    const [catalogues, setCatalogues] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [newCatalogueName, setNewCatalogueName] = useState('');
    const [newCatalogueInfo, setNewCatalogueInfo] = useState('');
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [currentCatalogueId, setCurrentCatalogueId] = useState(null);
    const [isDeleteConfirmOpen, setIsDeleteConfirmOpen] = useState(false);
    const [catalogueToDelete, setCatalogueToDelete] = useState(null);
    const [user, setUser] = useState(null);
    const getUser = () => {
        try {
            const user = JSON.parse(sessionStorage.getItem('user'));
            if (user && user.user) {
                setUser(user.user);
                console.log('User:', user.user);
            } else {
                console.error('用户信息未找到');
            }
        } catch (error) {
            console.error('解析用户信息时出错:', error);
        }
    };

    useEffect(() => {
        getUser();
}, []);
    if (user){
        console.log(user.user_detail_id);
    }else{
        console.log('no user');
    }

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

    const addCatalogue = async () => {
        try {
            const response = await axios.post(postCatalogueUrl(), { user_detail_id: user.user_detail_id,catalogue: newCatalogueName, info: newCatalogueInfo });
            setCatalogues([...catalogues, response.data]);
            setNewCatalogueName('');
            setNewCatalogueInfo('');
            setIsModalOpen(false);
        } catch (err) {
            setError(err);
        }
    };

    const deleteCatalogue = async () => {
        try {
            await axios.delete(deleteCatalogueUrl(catalogueToDelete));
            setCatalogues(catalogues.filter(catalogue => catalogue.id !== catalogueToDelete));
            setIsDeleteConfirmOpen(false);
            setCatalogueToDelete(null);
        } catch (err) {
            setError(err);
        }
    };

    const updateCatalogue = async () => {
        try {
            await axios.post(postUpdateCatalogueUrl(currentCatalogueId), {  catalogue: newCatalogueName, info: newCatalogueInfo });
            fetchCatalogues();
            setNewCatalogueName('');
            setNewCatalogueInfo('');
            setIsModalOpen(false);
        } catch (err) {
            setError(err);
        }
    };

    const openModal = (catalogue = null) => {
        if (catalogue) {
            setCurrentCatalogueId(catalogue.id);
            setNewCatalogueName(catalogue.catalogue);
            setNewCatalogueInfo(catalogue.info || '');
        } else {
            setCurrentCatalogueId(null);
            setNewCatalogueName('');
            setNewCatalogueInfo('');
        }
        setIsModalOpen(true);
    };

    const closeModal = () => {
        setIsModalOpen(false);
        setNewCatalogueName('');
        setNewCatalogueInfo('');
    };

    const openDeleteConfirm = (catalogueId) => {
        setCatalogueToDelete(catalogueId);
        setIsDeleteConfirmOpen(true);
    };

    const closeDeleteConfirm = () => {
        setIsDeleteConfirmOpen(false);
        setCatalogueToDelete(null);
    };

    if (loading) return <div className="text-center py-4">Loading...</div>;
    if (error) return <div className="text-center py-4 text-red-500">Error: {error.message}</div>;

    return (
        <ParentComponent>

            <div className="container mx-auto p-4">
                <h2 className="text-2xl font-bold mb-4">目录管理</h2>
                <button className="bg-blue-500 text-white px-4 py-2 rounded mb-4" onClick={() => openModal()}>添加新目录</button>

                <h2 className="text-xl font-semibold mb-2">目录列表</h2>
                <ul className="space-y-4">
                    {catalogues.map(catalogue => (
                        <li key={catalogue.id} className="p-4 border rounded shadow-sm">
                            <div>
                                <strong>名称:</strong> {catalogue.catalogue}
                            </div>
                            <div>
                                <strong>信息:</strong> {catalogue.info || '无'}
                            </div>
                            <div className="mt-2 space-x-2">
                                <button className="bg-red-500 text-white px-3 py-1 rounded" onClick={() => openDeleteConfirm(catalogue.id)}>删除</button>
                                <button className="bg-yellow-500 text-white px-3 py-1 rounded" onClick={() => openModal(catalogue)}>更新</button>
                            </div>
                        </li>
                    ))}
                </ul>

                {isModalOpen && (
                    <div className="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50">
                        <div className="bg-white p-6 rounded shadow-lg w-96 relative">
                            <span className="absolute top-2 right-2 text-2xl cursor-pointer" onClick={closeModal}>&times;</span>
                            <h2 className="text-xl font-bold mb-4">{currentCatalogueId ? '更新目录' : '添加新目录'}</h2>
                            <input
                                type="text"
                                value={newCatalogueName}
                                onChange={(e) => setNewCatalogueName(e.target.value)}
                                placeholder="目录名"
                                className="w-full p-2 mb-4 border rounded"
                            />
                            <input
                                type="text"
                                value={newCatalogueInfo}
                                onChange={(e) => setNewCatalogueInfo(e.target.value)}
                                placeholder="目录信息"
                                className="w-full p-2 mb-4 border rounded"
                            />
                            <div className="flex justify-end space-x-2">
                                <button className="bg-gray-500 text-white px-4 py-2 rounded" onClick={closeModal}>取消</button>
                                <button className="bg-blue-500 text-white px-4 py-2 rounded" onClick={currentCatalogueId ? updateCatalogue : addCatalogue}>
                                    {currentCatalogueId ? '更新' : '添加'}
                                </button>
                            </div>
                        </div>
                    </div>
                )}

                {isDeleteConfirmOpen && (
                    <div className="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50">
                        <div className="bg-white p-6 rounded shadow-lg w-96 relative">
                            <h2 className="text-xl font-bold mb-4">确认删除</h2>
                            <p>你确定要删除这个目录吗？此操作无法撤销。</p>
                            <div className="flex justify-end space-x-2 mt-4">
                                <button className="bg-gray-500 text-white px-4 py-2 rounded" onClick={closeDeleteConfirm}>取消</button>
                                <button className="bg-red-500 text-white px-4 py-2 rounded" onClick={deleteCatalogue}>删除</button>
                            </div>
                        </div>
                    </div>
                )}
            </div>
        </ParentComponent>

    );
};

export default CataloguesNew;