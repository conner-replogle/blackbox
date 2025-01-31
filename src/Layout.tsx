import { ThemeProvider } from "@/components/theme-provider";
import { SidebarInset, SidebarProvider, SidebarTrigger } from "./components/ui/sidebar";
import { AppSidebar } from "./components/app-sidebar";
import { Separator } from "./components/ui/separator";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator } from "./components/ui/breadcrumb";
import { Outlet, useLocation, useNavigate } from "react-router-dom";
import { useEffect } from "react";
import { check_status } from "./lib/api/database";


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
        <div className="relative flex min-h-screen flex-col">
        <SidebarProvider>
      <AppSidebar />
      <SidebarInset>
        
        <Outlet />
      </SidebarInset>
    </SidebarProvider>
        </div>
        {/* <TailwindIndicator /> */}
      </ThemeProvider>
    </div>
  );
}
