import React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import HomePage from '../pages/HomePage'
import DetailPage from '../pages/DetailPage';

const App = () => {
  return (
    <Router>
      <div className="App">
        <Routes>
          <Route exact path="/" element={<HomePage />} />
          <Route path="/detail/:url" element={<DetailPage />} />
        </Routes>
      </div>
    </Router>
  );
};

export default App;
