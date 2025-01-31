import Layout from "./Layout";
import HomePage from "@/routes/HomePage";
import { Navigate, Route, Routes } from "react-router-dom";
import Generate from "./routes/pgp/Generate";
import Message from "./routes/pgp/Message";
import PgpHomePage from "./routes/pgp/Pgp";
import UnlockPage from "./routes/unlock";
import Identities from "./routes/pgp/Identities";
import Contacts from "./routes/pgp/Contacts";



function App() {
  return (
      <Routes>

        <Route path="unlock" element={<UnlockPage />} />
        <Route path="/" element={<Layout />}>
          <Route index element={<HomePage />} />
          <Route path="pgp" element={<PgpHomePage/>}/>
          <Route path="pgp/generate" element={<Generate />} />
          <Route path="pgp/message" element={<Message />} />
          <Route path="pgp/identities" element={<Identities />} />
          <Route path="pgp/contacts" element={<Contacts/>}/>

          {/* Using path="*"" means "match anything", so this route
                acts like a catch-all for URLs that we don't have explicit
                routes for. */}
          <Route path="*" element={<NoMatch />} />
        </Route>
      </Routes>
      
  );
}

function NoMatch(){
  return <div>
    <Navigate to="/" />
    <h1>Not FOund</h1>
    
    </div>
}

export default App;
