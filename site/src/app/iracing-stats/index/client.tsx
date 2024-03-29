'use client'
 
import React, { useEffect, useState } from 'react'
import dynamic from 'next/dynamic'
import { BrowserRouter } from 'react-router-dom'
 
const App = dynamic(() => import('../../../App'), { ssr: false })
 
export function ClientOnly() {
  // Completely prevent server side rendering:
  // https://nextjs.org/docs/messages/react-hydration-error
  const [isClient, setIsClient] = useState(false)

  useEffect(() => {
    setIsClient(true)
  }, [])

  if (!isClient) {
    return null;
  }

  return (
    <React.StrictMode>
      <BrowserRouter>
        <App />
      </BrowserRouter>
    </React.StrictMode>
  )
}