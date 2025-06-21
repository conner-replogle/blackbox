import Layout from "@/layouts/MainLayout";
import HomePage from "@/routes/HomePage";
import { Navigate, Route, Routes } from "react-router";
import Generate from "./routes/pgp/Generate";
import Message from "./routes/pgp/Message";
import UnlockPage from "./routes/unlock";
import Identities from "./routes/pgp/Identities";
import Contacts from "./routes/pgp/Contacts";
import Wallet from "./routes/monero/wallet";
import { DashLayout } from "./layouts/DashLayout";



function App() {
  return (
    <Routes >
      <Route element={<Layout />}>

        <Route path="unlock" element={<UnlockPage />} />
        <Route path="/" element={<DashLayout />}>
          <Route index element={<HomePage />} />
          <Route path="pgp">
            <Route path="generate" element={<Generate />} />
            <Route path="message" element={<Message />} />
            <Route path="identities" element={<Identities />} />
            <Route path="contacts" element={<Contacts/>}/>
          </Route>
          <Route path="monero">
            <Route path="wallet" element={<Wallet/>}/>
          </Route>
          {/* Using path="*"" means "match anything", so this route
                acts like a catch-all for URLs that we don't have explicit
                routes for. */}
          <Route path="*" element={<NoMatch />} />
        </Route>
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
