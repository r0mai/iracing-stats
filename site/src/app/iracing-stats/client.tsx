'use client'
 
import React from 'react'
import dynamic from 'next/dynamic'
import { BrowserRouter } from 'react-router-dom'
 
const App = dynamic(() => import('../../App'), { ssr: false })
 
export function ClientOnly() {
  return (
    <React.StrictMode>
      <BrowserRouter>
        <App />
      </BrowserRouter>
    </React.StrictMode>
  )
}