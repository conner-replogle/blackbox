import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";
import { Outlet } from "react-router";
import { Header } from "@/components/header";

export function DashLayout() {
  return (
    <div className="relative flex min-h-screen flex-col">
      <SidebarProvider>
        <AppSidebar />
        <SidebarInset className="pb-4">

          <Header />
          <Outlet />
        </SidebarInset>


      </SidebarProvider>
    </div>
  );
}
