import { Gauge, type LucideIcon, Code, icons } from "lucide-react";

export type SiteConfig = typeof siteConfig;
export type Navigation = {
  icon: LucideIcon;
  name: string;
  href: string;
};

export const siteConfig = {
  title: "Leetcode Scaffold",
  description: "developed Template built with VisActor and Next.js",
};

export const navigations: Navigation[] = [
  {
    icon: Gauge,
    name: "Dashboard",
    href: "/",
  },
  {
    icon: Code,
    name: "Code PlayGround",
    href: "/playground",
  },
];
