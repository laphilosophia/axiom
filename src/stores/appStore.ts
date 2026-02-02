import { signal } from '@preact/signals'
import type { Document } from '../types/document'

// Global state signals
export const documentsSignal = signal<Document[]>([])
export const selectedDocumentSignal = signal<Document | null>(null)
export const isLoadingSignal = signal(false)
export const searchQuerySignal = signal('')
export const searchResultsSignal = signal<Document[]>([])
export const isSearchingSignal = signal(false)

// Store actions
export const useAppStore = () => ({
  // Getters
  get documents() {
    return documentsSignal.value
  },
  get selectedDocument() {
    return selectedDocumentSignal.value
  },
  get isLoading() {
    return isLoadingSignal.value
  },
  get searchQuery() {
    return searchQuerySignal.value
  },
  get searchResults() {
    return searchResultsSignal.value
  },
  get isSearching() {
    return isSearchingSignal.value
  },

  // Setters
  setDocuments: (docs: Document[]) => {
    documentsSignal.value = docs
  },
  setSelectedDocument: (doc: Document | null) => {
    selectedDocumentSignal.value = doc
  },
  setIsLoading: (loading: boolean) => {
    isLoadingSignal.value = loading
  },
  setSearchQuery: (query: string) => {
    searchQuerySignal.value = query
  },
  setSearchResults: (results: Document[]) => {
    searchResultsSignal.value = results
  },
  setIsSearching: (searching: boolean) => {
    isSearchingSignal.value = searching
  },
})
