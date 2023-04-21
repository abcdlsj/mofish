import React, { useEffect, useState } from "react";
import ReactHtmlParser from "html-react-parser";

const Detail = ({ url }) => {
  const [detail, setDetail] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const fetchDetail = async () => {
      setLoading(true);
      try {
        const response = await fetch(
          "http://localhost:3001/fetch/" + encodeURIComponent(url)
        );
        const data = await response.json();
        setDetail(data);
        setLoading(false);
      } catch (error) {
        console.log(error);
      }
    };

    fetchDetail();
  }, [url]);

  return (
    <div className="detail">
      {loading ? (
        <p>Loading detail...</p>
      ) : (
        <div>
          {detail && (
            <div>
              <h2 className="content-title">{detail.title}</h2>
              <div className="content-body">
                {ReactHtmlParser(detail.content)}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default Detail;
