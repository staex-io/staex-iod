<script>
export default {
  data() {
    return {
      address: '',
      device: null,
    }
  },
  methods: {
    async getDevice() {
      try {
        let res = await fetch(`/indexer/devices?address=${this.address}`, { method: 'GET' })
        switch (res.status) {
          case 200:
            break
          default:
            console.error(res)
            return
        }
        let data = await res.json()
        this.device = data[0]
      } catch (e) {
        console.error(e)
        return
      }
    },
  },
  mounted() {
    this.address = this.$route.params.address
    this.getDevice()
  },
}
</script>

<template>
  <div class="card" v-if="device">
    <div class="card-header">Device</div>
    <div class="card-content">
      <div class="card-field">
        <span class="card-field-label">Address</span>
        <div class="h-scroll-container">
          <span class="card-field-value">{{ device.address }}</span>
        </div>
      </div>
      <div class="card-field">
        <span class="card-field-label">Data type</span>
        <span class="card-field-value">{{ device.device.data_type }}</span>
      </div>
      <div class="card-field">
        <div class="h-scroll-container">
          <span class="card-field-label">Location</span>
          <span class="card-field-value">
            <a
              :href="`https://www.google.com/maps/place/${device.device.location}`"
              target="_blank"
            >
              {{ device.device.location }}
            </a>
          </span>
        </div>
      </div>
      <div class="card-field">
        <span class="card-field-label">Price to access data</span>
        <span class="card-field-value">{{ device.device.price_access }}</span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Price to pin data</span>
        <span class="card-field-value">{{ device.device.price_pin }}</span>
      </div>
      <div class="card-field">
        <span class="card-field-label">StaexMCC identifier</span>
        <span class="card-field-value">{{ device.device.staex_mcc_id }}</span>
      </div>
      <div class="card-field">
        <span class="card-field-label">MQTT topics</span>
        <span class="card-field-value">{{ device.device.mqtt_topics }}</span>
      </div>
      <hr />
      <div class="card-field">
        <div class="h-scroll-container">
          <span class="card-field-label">Additional</span>
          <span class="card-field-value">
            <pre>{{ JSON.stringify(device.device.additional, null, 4) }}</pre>
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
