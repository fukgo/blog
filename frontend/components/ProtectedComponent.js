// ParentComponent.js
import React, { useState, useEffect } from 'react';
import useAuth from './useAuth';
import Modal from './Modal';
import { loginUrl } from "@/api_list";

const ParentComponent = ({ children }) => {
    const [isAuthenticated, loading] = useAuth();
    const [isModalOpen, setIsModalOpen] = useState(false);

    console.log('isAuthenticated:', isAuthenticated);
    console.log('loading:', loading);

    useEffect(() => {
        if (!loading && !isAuthenticated) {
            setIsModalOpen(true);
            console.log('弹出模态框');
        }
    }, [loading, isAuthenticated]);

    if (loading) {
        return <div>Loading...</div>;
    }

    return (
        <div>
            {isModalOpen && (
                <Modal
                    isOpen={isModalOpen}
                />
            )}
            {!isModalOpen && isAuthenticated && children}
        </div>
    );
};

export default ParentComponent;