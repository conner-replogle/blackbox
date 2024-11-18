import React from "react";
import Layout from "./Layout";
import HomePage from "@/routes/HomePage";
import { Route, Routes } from "react-router-dom";
import Generate from "./routes/pgp/Generate";
import Message from "./routes/pgp/Message";
import PgpHomePage from "./routes/pgp/Pgp";
import UnlockPage from "./routes/unlock";



function App() {
  return (
      <Routes>

        <Route path="unlock" element={<UnlockPage />} />
        <Route path="/" element={<Layout />}>
          <Route index element={<HomePage />} />
          <Route path="pgp" element={<PgpHomePage/>}/>
          <Route path="pgp/generate" element={<Generate />} />
          <Route path="pgp/message" element={<Message />} />


          {/* Using path="*"" means "match anything", so this route
                acts like a catch-all for URLs that we don't have explicit
                routes for. */}
          {/* <Route path="*" element={<NoMatch />} /> */}
        </Route>
      </Routes>
      
  );
}

export default App;
