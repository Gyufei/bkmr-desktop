import { Button } from "./ui/button";

interface Props {
  currentPage: number;
  totalPages: number;
  onPageChange: (page: number) => void;
}

export default function Pagination({ currentPage, totalPages, onPageChange }: Props) {
  if (totalPages <= 1) return null;

  return (
    <div className="flex items-center justify-center gap-2 py-3">
      <Button
        variant="outline"
        size="sm"
        onClick={() => onPageChange(currentPage - 1)}
        disabled={currentPage <= 1}
      >
        上一页
      </Button>
      {Array.from({ length: totalPages }, (_, i) => i + 1)
        .filter((p) => p === 1 || p === totalPages || Math.abs(p - currentPage) <= 2)
        .map((p, idx, arr) => (
          <span key={p} className="flex items-center">
            {idx > 0 && arr[idx - 1] !== p - 1 && (
              <span className="px-1 text-text-secondary dark:text-text-dark-secondary">...</span>
            )}
            <Button
              variant={p === currentPage ? "default" : "outline"}
              size="sm"
              onClick={() => onPageChange(p)}
            >
              {p}
            </Button>
          </span>
        ))}
      <Button
        variant="outline"
        size="sm"
        onClick={() => onPageChange(currentPage + 1)}
        disabled={currentPage >= totalPages}
      >
        下一页
      </Button>
    </div>
  );
}
