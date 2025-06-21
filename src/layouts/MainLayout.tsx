import { ThemeProvider } from "@/components/theme-provider";

import { Outlet, useNavigate } from "react-router";
import { useEffect } from "react";
import { check_status } from "@/lib/api/database";


export default function RootLayout() {
  const navigate = useNavigate();
  useEffect(() => {
    check_status().then((status) => {
      console.log(status);
      if (!status){
        navigate("/unlock");
      }
    });
   
  },[]);


  return (
    <div className="min-h-screen bg-background font-sans antialiased">
      <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
        
        <Outlet />
      </ThemeProvider>
    </div>
  );
}