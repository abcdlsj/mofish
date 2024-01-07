import React, { useEffect, useState } from "react";
import { Link } from "react-router-dom";

const HomePage = () => {
  const [data, setData] = useState(false);

  const fetchData = () => {
    fetch("http://localhost:3001/crawl")
      .then((response) => response.json())
      .then((data) => {
        setData(data);
      })
      .catch((error) => console.log(error));
  };

  useEffect(() => {
    fetchData();
  }, []);

  return (
    <div className="App">
      <h1>Mo Fish</h1>
      {data && (
        <div className="api-data">
          {[
            { name: "Hupu", data: data.hupu },
            { name: "Hacker News", data: data.hackernews },
            { name: "Douban", data: data.douban },
          ].map((section) => (
            <div>
              <h2>{section.name}</h2>
                {section.data && section.data.map((item, index) => (
                <div key={`${section.name.toLowerCase()}-${index}`}>
                  <Link to={`/detail/${encodeURIComponent(item.url)}`}>
                    {item.title}
                  </Link>
                </div>
              ))}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default HomePage;
