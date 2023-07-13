import React, { useState } from 'react';
import { Link } from 'react-router-dom';

const HomePage = () => {
  const [data, setData] = useState(false);
  const [loading, setLoading] = useState(false);

  const fetchData = () => {
    setLoading(true);
    fetch('http://localhost:3001/crawl')
      .then(response => response.json())
      .then(data => {
        setData(data);
        setLoading(false);
      })
      .catch(error => console.log(error));
  };

  return (
    <div>
      <h1>JSON API App</h1>
      {loading ? (
        <p>Loading...</p>
      ) : (
        <div>
          {!data && <button onClick={fetchData}>Fetch Data</button>}
          {data && (
            <div className="api-data">
              <h2>Hupu</h2>
              {data.hupu.map(item => (
                <div key={item.url}>
                  <Link to={`/detail/${encodeURIComponent(item.url)}`}>{item.title}</Link>
                </div>
              ))}

              <h2>Hacker News</h2>
              {data.hackernews.map(item => (
                <div key={item.url}>
                  <Link to={`/detail/${encodeURIComponent(item.url)}`}>{item.title}</Link>
                </div>
              ))}

              <h2>Douban</h2>
              {data.douban.map(item => (
                <div key={item.url}>
                  <Link to={`/detail/${encodeURIComponent(item.url)}`}>{item.title}</Link>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default HomePage;
