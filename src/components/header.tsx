import { Separator } from "@radix-ui/react-separator";
import { Breadcrumb, BreadcrumbList, BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator } from "./ui/breadcrumb";
import { SidebarTrigger } from "./ui/sidebar";
import { useLocation } from "react-router";

export function Header(){

  const location = useLocation();
  const paths = location.pathname.split("/").map((path) => path.toLocaleUpperCase());
    return (
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
    )
}
