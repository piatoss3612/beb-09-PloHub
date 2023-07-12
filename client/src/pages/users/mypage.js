import React from 'react';
import axios from 'axios';
import cookie from 'cookie';
import { Header, Footer } from '../../Components/Reference'
import MyPage from '../../Components/User/MyPage';

const mypage = () => {

    return (
        <>
            <Header />
            <MyPage />
            <Footer />
        </>
    )
}

export default mypage;
