import React from 'react';
import { useParams } from 'react-router-dom';
import Detail from '../components/Detail';

const DetailPage = () => {
  const { url } = useParams();

  return (
    <div>
      <h1>Detail Page</h1>
      <a href="/">home</a>
      {' | '}
      <a href={url} target="_blank" rel="noopener noreferrer">source</a>
      <Detail url={url} />
    </div>
  );
};

export default DetailPage;
