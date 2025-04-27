// App.tsx
import { Routes, Route } from 'react-router-dom';
import { CartProvider } from './buyerInterface/context/CartContext';
import { Header } from './buyerInterface/components/Header';
import { CartDrawer } from './buyerInterface/components/Cart/CartDrawer';
import { Products } from './buyerInterface/pages/Products';
import { Checkout } from './buyerInterface/pages/Checkout.tsx';

export const App = () => (
  <CartProvider>
    <div className="min-h-screen flex flex-col">
      <Header />
      <main className="flex-1 bg-gray-50">
        <Routes>
          <Route path="/" element={<Products />} />
          <Route path="/products" element={<Products />} />
          <Route path="/checkout" element={<Checkout />} />
        </Routes>
      </main>
      <CartDrawer />
    </div>
  </CartProvider>
);