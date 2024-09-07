<template>
  <Section title="General">
    <table>
      <tbody>
        <tr>
          <td class="key">Request URL:</td>
          <td class="value">{{ requestDetails.uri }}</td>
        </tr>
        <tr>
          <td class="key">Request Method:</td>
          <td class="value">{{ requestDetails.method }}</td>
        </tr>
        <tr>
          <td class="key">Status Code:</td>
          <td class="value">{{ requestDetails.status_code }}</td>
        </tr>
      </tbody>
    </table>
  </Section>
  <Section title="Request Headers" />
  <Section title="Response Headers" />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRequestDetailsStore } from '../../store/request_details_store'
import Section from '../../components/Section.vue'

const requestDetailsStore = useRequestDetailsStore()
const requestDetails = computed(() => requestDetailsStore.activatedRequestDetails)
</script>

<style scoped>
.key {
  vertical-align: top;
  width: 200px;
}

.value {
  vertical-align: top;
}
</style>
