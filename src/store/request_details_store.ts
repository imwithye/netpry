import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface RequestDetails {
  uuid: string
  method: string
  uri: string,
  status_code: number | null;
  request_headers: { [key: string]: string },
  response_headers: { [key: string]: string },
  start_time: string
  end_time: string | null
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
