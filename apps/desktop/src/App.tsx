import Layout from './Layout';
import QueryProvider from '@/lib/query-provider';

export default function App() {
  return (
    <QueryProvider>
      <Layout />
    </QueryProvider>
  );
}
