import { ThemeProvider } from "@/components/theme-provider";
import { SidebarInset, SidebarProvider, SidebarTrigger } from "./components/ui/sidebar";
import { AppSidebar } from "./components/app-sidebar";
import { Separator } from "./components/ui/separator";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from "./components/ui/breadcrumb";
import { Navigate, Outlet, useLocation, useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";
import { listen } from '@tauri-apps/api/event';
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


  const location = useLocation();
  const paths = location.pathname.split("/").map((path) => path.toLocaleUpperCase());
  return (
    <div className="min-h-screen bg-background font-sans antialiased">
      <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
        <div className="relative flex min-h-screen flex-col">
        <SidebarProvider>
      <AppSidebar />
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12">
          <div className="flex items-center gap-2 px-4">
            <SidebarTrigger className="-ml-1" />
            <Separator orientation="vertical" className="mr-2 h-4" />
            <Breadcrumb>
              <BreadcrumbList>
                {
                  paths.map((path, index) => (
                    <>
                    <BreadcrumbItem className={index == paths.length-1 ? "" : "hidden md:block"} key={index}>
                      <BreadcrumbLink href="#">
                        {path}
                      </BreadcrumbLink>
                    </BreadcrumbItem>
                    {
                      (index < paths.length-1) && <BreadcrumbSeparator className="hidden md:block" />
                    }
                    </>
                  ))
                }
             
              </BreadcrumbList>
            </Breadcrumb>
          </div>
        </header>
        <Outlet />
      </SidebarInset>
    </SidebarProvider>
        </div>
        {/* <TailwindIndicator /> */}
      </ThemeProvider>
    </div>
  );
}
