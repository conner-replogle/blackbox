import { SidebarInset, SidebarProvider} from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";
import { Outlet } from "react-router";
import { Header } from "@/components/header";

export function DashLayout() {
  return (
<div className="relative flex min-h-screen flex-col">
    <SidebarProvider>
      <AppSidebar />
      <SidebarInset>
          <section>
      <Header />
      <Outlet />
      </section>

        
      </SidebarInset>
    </SidebarProvider>
</div>
    );
    }