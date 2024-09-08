import { defineStore } from 'pinia'
import { ref } from 'vue'
import Samples from '../assets/samples.json'

export interface RequestDetails {
  uuid: string
  method: string
  uri: string
  status_code: number | null
  request_headers: { [key: string]: string }
  response_headers: { [key: string]: string }
  request_body: string | null
  response_body: string | null
  start_time: string
  end_time: string | null
}

export const useRequestDetailsStore = defineStore('requestDetails', () => {
  const requestDetailsMap = new Map<string, number>()

  const activatedRequestDetails = ref<RequestDetails | null>(null)
  let samples = Samples as unknown as Array<RequestDetails>
  const requestDetailsList = ref<Array<RequestDetails>>([
    ...samples,
    ...samples,
    ...samples,
    ...samples
  ])

  function addRequestDetails(requestDetails: RequestDetails) {
    if (!requestDetails) return

    if (!requestDetailsMap.has(requestDetails.uuid)) {
      requestDetailsMap.set(requestDetails.uuid, requestDetailsList.value.length)
      requestDetailsList.value.push(requestDetails)
      return
    }
    const idx = requestDetailsMap.get(requestDetails.uuid) as number
    requestDetailsList.value[idx] = requestDetails
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
