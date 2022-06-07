import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";

import Home from "./pages/Home";
import Board from "./pages/Board";

const App = () => (
  <main>
    <Router>
      <Routes>
        <Route path="/board/:boardId" element={<Board />} />
        <Route path="/" element={<Home />} />
      </Routes>
    </Router>
  </main>
);

export default App;