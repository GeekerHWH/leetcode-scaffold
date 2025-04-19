import Link from "next/link";

export default function MadeBy() {
  return (
    <Link
      href="https://geekerhwh.pages.dev/"
      target="_blank"
      className="relative my-2 flex flex-col items-center justify-center gap-y-2 px-4 py-4"
    >
      <div className="dot-matrix absolute left-0 top-0 -z-10 h-full w-full" />
      <span className="text-xs text-muted-foreground">Made by</span>
      <div className="flex items-center space-x-2">
        <span className="text-md text-accent-foreground">GeekerHWH</span>
      </div>
    </Link>
  );
}
