//ProtectedComponent.js
import React, { useState, useEffect } from 'react';
import useAuth from './useAuth';
import Modal from './Modal';

const ParentComponent = ({ children }) => {
    const { isAuthenticated, loading } = useAuth();
    const [isModalOpen, setIsModalOpen] = useState(false);

    const handleClose = () => {
        setIsModalOpen(false);
        window.location.href = '/';
    };

    const handleLogin = () => {
        const redirectUrl = encodeURIComponent(localStorage.getItem('redirect_url'));
        window.location.href = `http://127.0.0.1:8001/auth/login?redirect=${redirectUrl}`;
    };

    useEffect(() => {
        if (!loading && !isAuthenticated) {
            setIsModalOpen(true);
        }
    }, [loading, isAuthenticated]);

    if (loading) {
        return <div>Loading...</div>;
    }

    return (
        <div>
            {isModalOpen && <Modal isOpen={isModalOpen} onClose={handleClose} onLogin={handleLogin} />}
            {isAuthenticated && children}
        </div>
    );
};

export default ParentComponent;