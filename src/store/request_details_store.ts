import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface RequestDetails {
  method: string
  url: string
}

export const useRequestDetailsStore = defineStore('requestDetails', () => {
  const activatedRequestDetails = ref<RequestDetails | null>(null)
  const requestDetailsList = ref<Array<RequestDetails>>([])

  function addRequestDetails(requestDetails: RequestDetails) {
    if (!requestDetails) return
    requestDetailsList.value.push(requestDetails)
  }

  function setActiveRequestDetails(requestDetails: RequestDetails | null) {
    activatedRequestDetails.value = requestDetails
  }

  return {
    activatedRequestDetails,
    requestDetailsList,
    addRequestDetails,
    setActiveRequestDetails
  }
})
