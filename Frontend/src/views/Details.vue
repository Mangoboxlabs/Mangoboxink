<template>
  <div class="DetailsView">
    <detailHeader></detailHeader>
    <div class="DetailsView-content">
      <div class="left-content">
        <div class="row">
          <div class="name">
            VOLUME
          </div>
          <div class="value">
            {{ volume }}
          </div>
        </div>
        <div class="row">
          <div class="name">
            IN TREASURY
          </div>
          <div class="value">
            {{ volume }}
          </div>
        </div>
        <div class="row">
          <div class="name">
            DISTRIBUTED
          </div>
          <div class="value">
            100% OVERFLOW
          </div>
        </div>
        <!--      <el-progress :text-inside="true" :stroke-width="26" :percentage="70"></el-progress>-->
        <div class="row">
          <div>
            IN WALLET
          </div>
          <div>
            0
          </div>
        </div>
        <div id="coin-chart"></div>
        <h4 class="sub-title">
          Tokens
          <div class="mangobox-button" @click="isShowCreate=true">
            Create
          </div>
          <div class="mangobox-button" @click="mintFor">
            mintFor
          </div>
        </h4>
        <div class="row">
          <div class="name">
            Total supply
          </div>
          <div class="value">
            {{ tokenSupply }}tokens
          </div>
        </div>
        <div class="row">
          <div class="name">
            Your balance
          </div>
          <div class="value">
            {{ tokenBalance }} tokens
          </div>
        </div>
        <h4 class="sub-title">
          Funding cycle
        </h4>
        <strong style="font-size: 1.5em">
          10,308 TOKENS RESERVED
        </strong>
      </div>
      <div class="right-content">
        <div class="row">
          <div class="input-box">
            <input type="text">
          </div>
          <div class="mangobox-button" style="padding: 0 30px">
            ADD
          </div>
        </div>
        <div class="row">
          <strong>
            Receive 29,779 tokens/1 ETH
          </strong>
        </div>
        <h3 class="sub-title">
          Activity
        </h3>
        <div class="paid-list">
          <div class="paid-item">
            <div class="row paid-item-header">
              <div>
                Paid
              </div>
              <div>
                16 hours ago
              </div>
            </div>
            <div class="row paid-item-content">
              <div class="value">
                $ 0.0016
              </div>
              <div>
                0x54bc...cf57ce
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="create-token-dialog" v-show="isShowCreate">
      <div class="mask"></div>
      <div class="content  animate__animated  animate__bounceInLeft" @click.stop>
        <h1>Create Token</h1>
        <input type="text" v-model="tokenName" placeholder="Token Name">
        <input type="text" v-model="tokenSymbol" aria-placeholder="Token Symbol">
        <div class="operate-btns">
          <div class="mangobox-button" @click="issueTokenFor">
            Create
          </div>
          <div class="mangobox-button" @click="isShowCreate=false">
            Cancel
          </div>
        </div>

      </div>
    </div>
  </div>
</template>

<script>
import {Chart} from '@antv/g2';
import detailHeader from "../components/detail-header"

function findMaxMin(data) {
  let maxValue = 0;
  let minValue = 50000;
  let maxObj = null;
  let minObj = null;
  for (const d of data) {
    if (d.Close > maxValue) {
      maxValue = d.Close;
      maxObj = d;
    }
    if (d.Close < minValue) {
      minValue = d.Close;
      minObj = d;
    }
  }
  return {max: maxObj, min: minObj};
}

export default {
  name: "DetailsView",
  components: {
    detailHeader
  },
  data() {
    return {
      isShowCreate:false,
      volume: 0,
      tokenSupply: 0,
      tokenBalance: 0,
      projectId: null,
      tokenName: "as",
      tokenSymbol: "aAASym"
    }
  },
  methods: {
    getBalanceOf(id) {
      return this.$store.dispatch("MBSingleTokenPaymentTerminalStore/getBalanceOf", {
        _account: "5FP3znY5SuthDe6xNE7oMyV5TPXdCsV6RBCNRRdCfRFdGuuy",
        _projectId: id
      })
    },
    balanceOf() {

      this.$store.dispatch("MBTokenStore/balanceOf", {
        _holder: "5DyQtzR89WGjAchokxH5Ntc2mxC9ruaUGi4nKYZCSq6VUMg1", _projectId: this.projectId
      }).then(res => {
        this.volume = res
      })
    },
    issueTokenFor() {
      return this.$store.dispatch("MBController/issueTokenFor", {
        _projectId: this.projectId, _name: this.tokenName, _symbol: this.tokenSymbol
      })
    },
    mintFor() {
      this.$store.dispatch("MBTokenStore/mintFor", {
        _holder: "5DyQtzR89WGjAchokxH5Ntc2mxC9ruaUGi4nKYZCSq6VUMg1", _projectId: this.projectId, _amount: 100000000000, _preferClaimedTokens: false
      })
    }
  },
  created() {
    this.projectId = this.$route.query.id
    console.log(    this.projectId
    )
    this.getBalanceOf(this.projectId).then(res => {
      this.volume = res
    })
    this.balanceOf()
  },
  mounted() {
    const data = [
      {
        "Date": "04.01.2016",
        "Close": 126.12
      },
      {
        "Date": "05.01.2016",
        "Close": 125.688
      },
      {
        "Date": "06.01.2016",
        "Close": 119.704
      },
      {
        "Date": "07.01.2016",
        "Close": 120.19
      },
      {
        "Date": "08.01.2016",
        "Close": 121.157
      },
      {
        "Date": "11.01.2016",
        "Close": 117
      },
      {
        "Date": "12.01.2016",
        "Close": 111.487
      },
      {
        "Date": "13.01.2016",
        "Close": 122
      },
      {
        "Date": "14.01.2016",
        "Close": 117.76
      },
      {
        "Date": "15.01.2016",
        "Close": 114.397
      },
      {
        "Date": "18.01.2016",
        "Close": 116.373
      },
      {
        "Date": "19.01.2016",
        "Close": 120.547
      },
      {
        "Date": "20.01.2016",
        "Close": 113.733
      },
      {
        "Date": "21.01.2016",
        "Close": 114.098
      },
      {
        "Date": "22.01.2016",
        "Close": 123.833
      },
      {
        "Date": "25.01.2016",
        "Close": 125
      },
      {
        "Date": "26.01.2016",
        "Close": 124.866
      },
      {
        "Date": "27.01.2016",
        "Close": 120.264
      },
      {
        "Date": "28.01.2016",
        "Close": 122.296
      },
      {
        "Date": "29.01.2016",
        "Close": 124.502
      },
      {
        "Date": "01.02.2016",
        "Close": 127.936
      },
      {
        "Date": "02.02.2016",
        "Close": 132.513
      },
      {
        "Date": "03.02.2016",
        "Close": 129.95
      },
      {
        "Date": "04.02.2016",
        "Close": 129.275
      },
      {
        "Date": "05.02.2016",
        "Close": 127.898
      },
      {
        "Date": "08.02.2016",
        "Close": 134.9
      },
      {
        "Date": "09.02.2016",
        "Close": 122.819
      },
      {
        "Date": "10.02.2016",
        "Close": 120.108
      },
      {
        "Date": "11.02.2016",
        "Close": 119.447
      },
      {
        "Date": "12.02.2016",
        "Close": 117.8
      },
      {
        "Date": "15.02.2016",
        "Close": 122.8
      },
      {
        "Date": "16.02.2016",
        "Close": 121.865
      },
      {
        "Date": "17.02.2016",
        "Close": 126.3
      },
      {
        "Date": "18.02.2016",
        "Close": 128.259
      },
      {
        "Date": "19.02.2016",
        "Close": 125.724
      },
      {
        "Date": "22.02.2016",
        "Close": 130
      },
      {
        "Date": "23.02.2016",
        "Close": 129.948
      },
      {
        "Date": "24.02.2016",
        "Close": 132.5
      },
      {
        "Date": "25.02.2016",
        "Close": 128.08
      },
      {
        "Date": "26.02.2016",
        "Close": 122
      },
      {
        "Date": "29.02.2016",
        "Close": 122
      },
      {
        "Date": "01.03.2016",
        "Close": 123.449
      },
      {
        "Date": "02.03.2016",
        "Close": 130.139
      },
      {
        "Date": "03.03.2016",
        "Close": 132
      },
      {
        "Date": "04.03.2016",
        "Close": 135
      },
      {
        "Date": "07.03.2016",
        "Close": 123.905
      },
      {
        "Date": "08.03.2016",
        "Close": 125.155
      },
      {
        "Date": "09.03.2016",
        "Close": 126
      },
      {
        "Date": "10.03.2016",
        "Close": 126.778
      },
      {
        "Date": "11.03.2016",
        "Close": 129.656
      },
      {
        "Date": "14.03.2016",
        "Close": 127.64
      },
      {
        "Date": "15.03.2016",
        "Close": 124.786
      },
      {
        "Date": "16.03.2016",
        "Close": 124.469
      },
      {
        "Date": "17.03.2016",
        "Close": 123.5
      },
      {
        "Date": "18.03.2016",
        "Close": 124.061
      },
      {
        "Date": "21.03.2016",
        "Close": 123.5
      },
      {
        "Date": "22.03.2016",
        "Close": 129.002
      },
      {
        "Date": "23.03.2016",
        "Close": 129
      },
      {
        "Date": "24.03.2016",
        "Close": 131.31
      },
      {
        "Date": "29.03.2016",
        "Close": 133.354
      },
      {
        "Date": "30.03.2016",
        "Close": 129.298
      },
      {
        "Date": "31.03.2016",
        "Close": 127.4
      },
      {
        "Date": "01.04.2016",
        "Close": 122.376
      },
      {
        "Date": "04.04.2016",
        "Close": 119.467
      },
      {
        "Date": "05.04.2016",
        "Close": 120.695
      },
      {
        "Date": "06.04.2016",
        "Close": 118.725
      },
      {
        "Date": "07.04.2016",
        "Close": 127.539
      },
      {
        "Date": "08.04.2016",
        "Close": 129.8
      },
      {
        "Date": "11.04.2016",
        "Close": 129.5
      },
      {
        "Date": "12.04.2016",
        "Close": 134.465
      },
      {
        "Date": "13.04.2016",
        "Close": 133
      },
      {
        "Date": "14.04.2016",
        "Close": 137.35
      },
      {
        "Date": "15.04.2016",
        "Close": 137.2
      },
      {
        "Date": "18.04.2016",
        "Close": 132.611
      },
      {
        "Date": "19.04.2016",
        "Close": 135.479
      },
      {
        "Date": "20.04.2016",
        "Close": 139.05
      },
      {
        "Date": "21.04.2016",
        "Close": 142
      },
      {
        "Date": "22.04.2016",
        "Close": 135.761
      },
      {
        "Date": "25.04.2016",
        "Close": 136.174
      },
      {
        "Date": "26.04.2016",
        "Close": 134.782
      },
      {
        "Date": "27.04.2016",
        "Close": 128
      },
      {
        "Date": "28.04.2016",
        "Close": 121.5
      },
      {
        "Date": "29.04.2016",
        "Close": 120
      },
      {
        "Date": "02.05.2016",
        "Close": 123.966
      },
      {
        "Date": "03.05.2016",
        "Close": 122.538
      },
      {
        "Date": "04.05.2016",
        "Close": 120
      },
      {
        "Date": "05.05.2016",
        "Close": 120.21
      },
      {
        "Date": "06.05.2016",
        "Close": 121.01
      },
      {
        "Date": "09.05.2016",
        "Close": 125.4
      },
      {
        "Date": "10.05.2016",
        "Close": 120.622
      },
      {
        "Date": "11.05.2016",
        "Close": 123.85
      },
      {
        "Date": "12.05.2016",
        "Close": 122.963
      },
      {
        "Date": "13.05.2016",
        "Close": 126
      },
      {
        "Date": "17.05.2016",
        "Close": 130
      },
      {
        "Date": "18.05.2016",
        "Close": 128.845
      },
      {
        "Date": "19.05.2016",
        "Close": 130.17
      },
      {
        "Date": "20.05.2016",
        "Close": 129.741
      },
      {
        "Date": "23.05.2016",
        "Close": 129.668
      },
      {
        "Date": "24.05.2016",
        "Close": 126.886
      },
      {
        "Date": "25.05.2016",
        "Close": 128.239
      },
      {
        "Date": "26.05.2016",
        "Close": 127.239
      },
      {
        "Date": "27.05.2016",
        "Close": 127.457
      },
      {
        "Date": "30.05.2016",
        "Close": 127.37
      },
      {
        "Date": "31.05.2016",
        "Close": 130.756
      },
      {
        "Date": "01.06.2016",
        "Close": 133.232
      },
      {
        "Date": "02.06.2016",
        "Close": 126.47
      },
      {
        "Date": "03.06.2016",
        "Close": 126.385
      },
      {
        "Date": "06.06.2016",
        "Close": 128.331
      },
      {
        "Date": "07.06.2016",
        "Close": 130.914
      },
      {
        "Date": "08.06.2016",
        "Close": 133
      },
      {
        "Date": "09.06.2016",
        "Close": 133.041
      },
      {
        "Date": "10.06.2016",
        "Close": 133.041
      },
      {
        "Date": "13.06.2016",
        "Close": 129
      },
      {
        "Date": "14.06.2016",
        "Close": 129.166
      },
      {
        "Date": "15.06.2016",
        "Close": 124.687
      },
      {
        "Date": "16.06.2016",
        "Close": 122.77
      },
      {
        "Date": "17.06.2016",
        "Close": 126.461
      },
      {
        "Date": "20.06.2016",
        "Close": 127
      },
      {
        "Date": "21.06.2016",
        "Close": 125.594
      },
      {
        "Date": "22.06.2016",
        "Close": 127.438
      },
      {
        "Date": "23.06.2016",
        "Close": 124.44
      },
      {
        "Date": "24.06.2016",
        "Close": 122.131
      },
      {
        "Date": "27.06.2016",
        "Close": 120.53
      },
      {
        "Date": "28.06.2016",
        "Close": 120.296
      },
      {
        "Date": "29.06.2016",
        "Close": 125.877
      },
      {
        "Date": "30.06.2016",
        "Close": 126.404
      },
      {
        "Date": "01.07.2016",
        "Close": 130.147
      },
      {
        "Date": "04.07.2016",
        "Close": 129.152
      },
      {
        "Date": "05.07.2016",
        "Close": 125.719
      },
      {
        "Date": "06.07.2016",
        "Close": 129.269
      },
      {
        "Date": "07.07.2016",
        "Close": 131.713
      },
      {
        "Date": "08.07.2016",
        "Close": 146.969
      },
      {
        "Date": "11.07.2016",
        "Close": 201.7
      },
      {
        "Date": "12.07.2016",
        "Close": 202.01
      },
      {
        "Date": "13.07.2016",
        "Close": 195.45
      },
      {
        "Date": "14.07.2016",
        "Close": 220.49
      },
      {
        "Date": "15.07.2016",
        "Close": 238.07
      },
      {
        "Date": "18.07.2016",
        "Close": 270.282
      },
      {
        "Date": "19.07.2016",
        "Close": 258.39
      },
      {
        "Date": "20.07.2016",
        "Close": 243.1
      },
      {
        "Date": "21.07.2016",
        "Close": 237
      },
      {
        "Date": "22.07.2016",
        "Close": 208
      },
      {
        "Date": "25.07.2016",
        "Close": 188.02
      },
      {
        "Date": "26.07.2016",
        "Close": 198.65
      },
      {
        "Date": "27.07.2016",
        "Close": 188
      },
      {
        "Date": "28.07.2016",
        "Close": 180.99
      },
      {
        "Date": "29.07.2016",
        "Close": 186
      },
      {
        "Date": "01.08.2016",
        "Close": 181
      },
      {
        "Date": "02.08.2016",
        "Close": 179.33
      },
      {
        "Date": "03.08.2016",
        "Close": 186
      },
      {
        "Date": "04.08.2016",
        "Close": 187.212
      },
      {
        "Date": "05.08.2016",
        "Close": 184.5
      },
      {
        "Date": "08.08.2016",
        "Close": 189.5
      },
      {
        "Date": "09.08.2016",
        "Close": 202.5
      },
      {
        "Date": "10.08.2016",
        "Close": 202.9
      },
      {
        "Date": "11.08.2016",
        "Close": 200.5
      },
      {
        "Date": "12.08.2016",
        "Close": 195.98
      },
      {
        "Date": "15.08.2016",
        "Close": 196
      },
      {
        "Date": "16.08.2016",
        "Close": 192
      },
      {
        "Date": "17.08.2016",
        "Close": 196.3
      },
      {
        "Date": "18.08.2016",
        "Close": 200.25
      },
      {
        "Date": "19.08.2016",
        "Close": 195
      },
      {
        "Date": "22.08.2016",
        "Close": 200.01
      },
      {
        "Date": "23.08.2016",
        "Close": 199.99
      },
      {
        "Date": "24.08.2016",
        "Close": 195.65
      },
      {
        "Date": "25.08.2016",
        "Close": 195.074
      },
      {
        "Date": "26.08.2016",
        "Close": 191.98
      },
      {
        "Date": "29.08.2016",
        "Close": 193.8
      },
      {
        "Date": "30.08.2016",
        "Close": 192.45
      },
      {
        "Date": "31.08.2016",
        "Close": 194
      },
      {
        "Date": "01.09.2016",
        "Close": 199.1
      },
      {
        "Date": "02.09.2016",
        "Close": 206.21
      },
      {
        "Date": "05.09.2016",
        "Close": 201.98
      },
      {
        "Date": "06.09.2016",
        "Close": 201.1
      },
      {
        "Date": "07.09.2016",
        "Close": 245.25
      },
      {
        "Date": "08.09.2016",
        "Close": 238.01
      },
      {
        "Date": "09.09.2016",
        "Close": 235
      },
      {
        "Date": "12.09.2016",
        "Close": 237.98
      },
      {
        "Date": "13.09.2016",
        "Close": 223.55
      },
      {
        "Date": "14.09.2016",
        "Close": 222.05
      },
      {
        "Date": "15.09.2016",
        "Close": 233.26
      },
      {
        "Date": "16.09.2016",
        "Close": 234
      },
      {
        "Date": "19.09.2016",
        "Close": 235.81
      },
      {
        "Date": "20.09.2016",
        "Close": 239.8
      },
      {
        "Date": "21.09.2016",
        "Close": 238.1
      },
      {
        "Date": "22.09.2016",
        "Close": 241.35
      },
      {
        "Date": "23.09.2016",
        "Close": 237.8
      },
      {
        "Date": "26.09.2016",
        "Close": 232.36
      },
      {
        "Date": "27.09.2016",
        "Close": 238.95
      },
      {
        "Date": "28.09.2016",
        "Close": 237.52
      },
      {
        "Date": "29.09.2016",
        "Close": 237.5
      },
      {
        "Date": "30.09.2016",
        "Close": 235
      },
      {
        "Date": "04.10.2016",
        "Close": 230.82
      },
      {
        "Date": "05.10.2016",
        "Close": 233.5
      },
      {
        "Date": "06.10.2016",
        "Close": 227.094
      },
      {
        "Date": "07.10.2016",
        "Close": 227.783
      },
      {
        "Date": "10.10.2016",
        "Close": 232.55
      },
      {
        "Date": "11.10.2016",
        "Close": 229
      },
      {
        "Date": "12.10.2016",
        "Close": 225.6
      },
      {
        "Date": "13.10.2016",
        "Close": 228.45
      },
      {
        "Date": "14.10.2016",
        "Close": 230.19
      },
      {
        "Date": "17.10.2016",
        "Close": 228.375
      },
      {
        "Date": "18.10.2016",
        "Close": 228.05
      },
      {
        "Date": "19.10.2016",
        "Close": 232.85
      },
      {
        "Date": "20.10.2016",
        "Close": 238.45
      },
      {
        "Date": "21.10.2016",
        "Close": 225.05
      },
      {
        "Date": "24.10.2016",
        "Close": 214.747
      },
      {
        "Date": "25.10.2016",
        "Close": 218.5
      },
      {
        "Date": "26.10.2016",
        "Close": 207.95
      },
      {
        "Date": "27.10.2016",
        "Close": 215.764
      },
      {
        "Date": "28.10.2016",
        "Close": 222.98
      },
      {
        "Date": "31.10.2016",
        "Close": 219.458
      },
      {
        "Date": "01.11.2016",
        "Close": 218.522
      },
      {
        "Date": "02.11.2016",
        "Close": 220.714
      },
      {
        "Date": "03.11.2016",
        "Close": 216.305
      },
      {
        "Date": "04.11.2016",
        "Close": 216.25
      },
      {
        "Date": "07.11.2016",
        "Close": 214.85
      },
      {
        "Date": "08.11.2016",
        "Close": 213.45
      },
      {
        "Date": "09.11.2016",
        "Close": 206.045
      },
      {
        "Date": "10.11.2016",
        "Close": 211
      },
      {
        "Date": "11.11.2016",
        "Close": 210.954
      },
      {
        "Date": "14.11.2016",
        "Close": 215.16
      },
      {
        "Date": "15.11.2016",
        "Close": 211.49
      },
      {
        "Date": "16.11.2016",
        "Close": 218
      },
      {
        "Date": "17.11.2016",
        "Close": 223.2
      },
      {
        "Date": "18.11.2016",
        "Close": 229.5
      },
      {
        "Date": "21.11.2016",
        "Close": 233.949
      },
      {
        "Date": "22.11.2016",
        "Close": 237.3
      },
      {
        "Date": "23.11.2016",
        "Close": 241.182
      },
      {
        "Date": "24.11.2016",
        "Close": 234.479
      },
      {
        "Date": "25.11.2016",
        "Close": 232.55
      },
      {
        "Date": "28.11.2016",
        "Close": 238.5
      },
      {
        "Date": "29.11.2016",
        "Close": 233
      },
      {
        "Date": "30.11.2016",
        "Close": 234
      },
      {
        "Date": "01.12.2016",
        "Close": 230.51
      },
      {
        "Date": "02.12.2016",
        "Close": 222.938
      },
      {
        "Date": "05.12.2016",
        "Close": 225.8
      },
      {
        "Date": "06.12.2016",
        "Close": 231
      },
      {
        "Date": "07.12.2016",
        "Close": 232.849
      },
      {
        "Date": "08.12.2016",
        "Close": 234.473
      },
      {
        "Date": "09.12.2016",
        "Close": 241.82
      },
      {
        "Date": "12.12.2016",
        "Close": 242.501
      },
      {
        "Date": "13.12.2016",
        "Close": 234.98
      },
      {
        "Date": "14.12.2016",
        "Close": 229.1
      },
      {
        "Date": "15.12.2016",
        "Close": 227.999
      },
      {
        "Date": "16.12.2016",
        "Close": 207.995
      },
      {
        "Date": "19.12.2016",
        "Close": 201.7
      },
      {
        "Date": "20.12.2016",
        "Close": 205.97
      },
      {
        "Date": "21.12.2016",
        "Close": 197.799
      },
      {
        "Date": "22.12.2016",
        "Close": 189.89
      },
      {
        "Date": "23.12.2016",
        "Close": 190
      },
      {
        "Date": "27.12.2016",
        "Close": 201
      },
      {
        "Date": "28.12.2016",
        "Close": 199.95
      },
      {
        "Date": "29.12.2016",
        "Close": 200.9
      },
      {
        "Date": "30.12.2016",
        "Close": 199
      },
      {
        "Date": "02.01.2017",
        "Close": 199
      },
      {
        "Date": "03.01.2017",
        "Close": 202.8
      },
      {
        "Date": "04.01.2017",
        "Close": 201.6
      },
      {
        "Date": "05.01.2017",
        "Close": 195.1
      },
      {
        "Date": "06.01.2017",
        "Close": 196
      },
      {
        "Date": "09.01.2017",
        "Close": 197.98
      },
      {
        "Date": "10.01.2017",
        "Close": 203
      },
      {
        "Date": "11.01.2017",
        "Close": 207
      },
      {
        "Date": "12.01.2017",
        "Close": 207.9
      },
      {
        "Date": "13.01.2017",
        "Close": 197.95
      },
      {
        "Date": "16.01.2017",
        "Close": 190.55
      },
      {
        "Date": "17.01.2017",
        "Close": 195
      },
      {
        "Date": "18.01.2017",
        "Close": 195.201
      },
      {
        "Date": "19.01.2017",
        "Close": 194.05
      },
      {
        "Date": "20.01.2017",
        "Close": 194
      },
      {
        "Date": "23.01.2017",
        "Close": 194
      },
      {
        "Date": "24.01.2017",
        "Close": 192.05
      },
      {
        "Date": "25.01.2017",
        "Close": 192.5
      },
      {
        "Date": "26.01.2017",
        "Close": 194.98
      },
      {
        "Date": "27.01.2017",
        "Close": 191.32
      },
      {
        "Date": "30.01.2017",
        "Close": 193
      },
      {
        "Date": "31.01.2017",
        "Close": 187
      },
      {
        "Date": "01.02.2017",
        "Close": 187
      },
      {
        "Date": "02.02.2017",
        "Close": 187.45
      },
      {
        "Date": "03.02.2017",
        "Close": 196
      },
      {
        "Date": "06.02.2017",
        "Close": 196.9
      },
      {
        "Date": "07.02.2017",
        "Close": 196.75
      },
      {
        "Date": "08.02.2017",
        "Close": 192
      },
      {
        "Date": "09.02.2017",
        "Close": 195
      },
      {
        "Date": "10.02.2017",
        "Close": 196
      },
      {
        "Date": "13.02.2017",
        "Close": 198.35
      },
      {
        "Date": "14.02.2017",
        "Close": 192
      },
      {
        "Date": "15.02.2017",
        "Close": 191.9
      },
      {
        "Date": "16.02.2017",
        "Close": 193.5
      },
      {
        "Date": "17.02.2017",
        "Close": 194.85
      },
      {
        "Date": "20.02.2017",
        "Close": 194.8
      },
      {
        "Date": "21.02.2017",
        "Close": 193.05
      },
      {
        "Date": "22.02.2017",
        "Close": 194.85
      },
      {
        "Date": "23.02.2017",
        "Close": 195.9
      },
      {
        "Date": "24.02.2017",
        "Close": 197.5
      },
      {
        "Date": "27.02.2017",
        "Close": 197.5
      },
      {
        "Date": "28.02.2017",
        "Close": 199
      },
      {
        "Date": "01.03.2017",
        "Close": 197
      },
      {
        "Date": "02.03.2017",
        "Close": 192
      },
      {
        "Date": "03.03.2017",
        "Close": 199
      },
      {
        "Date": "06.03.2017",
        "Close": 200.5
      },
      {
        "Date": "07.03.2017",
        "Close": 207.5
      },
      {
        "Date": "08.03.2017",
        "Close": 206
      },
      {
        "Date": "09.03.2017",
        "Close": 200.1
      },
      {
        "Date": "10.03.2017",
        "Close": 200.05
      },
      {
        "Date": "13.03.2017",
        "Close": 205.55
      },
      {
        "Date": "14.03.2017",
        "Close": 202
      },
      {
        "Date": "15.03.2017",
        "Close": 202.05
      },
      {
        "Date": "16.03.2017",
        "Close": 207
      },
      {
        "Date": "17.03.2017",
        "Close": 216.95
      },
      {
        "Date": "20.03.2017",
        "Close": 219.8
      },
      {
        "Date": "21.03.2017",
        "Close": 215.05
      },
      {
        "Date": "22.03.2017",
        "Close": 227.998
      },
      {
        "Date": "23.03.2017",
        "Close": 229
      },
      {
        "Date": "24.03.2017",
        "Close": 227.437
      },
      {
        "Date": "27.03.2017",
        "Close": 221.679
      },
      {
        "Date": "28.03.2017",
        "Close": 220
      },
      {
        "Date": "29.03.2017",
        "Close": 222.8
      },
      {
        "Date": "30.03.2017",
        "Close": 217.092
      },
      {
        "Date": "31.03.2017",
        "Close": 215.03
      },
      {
        "Date": "03.04.2017",
        "Close": 224.9
      },
      {
        "Date": "04.04.2017",
        "Close": 220
      },
      {
        "Date": "05.04.2017",
        "Close": 218.075
      },
      {
        "Date": "06.04.2017",
        "Close": 221.1
      },
      {
        "Date": "07.04.2017",
        "Close": 219.49
      },
      {
        "Date": "10.04.2017",
        "Close": 220
      },
      {
        "Date": "11.04.2017",
        "Close": 216.15
      },
      {
        "Date": "12.04.2017",
        "Close": 216.2
      },
      {
        "Date": "13.04.2017",
        "Close": 216.995
      },
      {
        "Date": "18.04.2017",
        "Close": 227.58
      },
      {
        "Date": "19.04.2017",
        "Close": 232.6
      },
      {
        "Date": "20.04.2017",
        "Close": 231.086
      },
      {
        "Date": "21.04.2017",
        "Close": 229.673
      },
      {
        "Date": "24.04.2017",
        "Close": 230.1
      },
      {
        "Date": "25.04.2017",
        "Close": 232.98
      },
      {
        "Date": "26.04.2017",
        "Close": 228.15
      },
      {
        "Date": "27.04.2017",
        "Close": 223.96
      },
      {
        "Date": "28.04.2017",
        "Close": 231
      },
      {
        "Date": "02.05.2017",
        "Close": 229.715
      },
      {
        "Date": "03.05.2017",
        "Close": 229.5
      },
      {
        "Date": "04.05.2017",
        "Close": 230
      },
      {
        "Date": "05.05.2017",
        "Close": 234.1
      },
      {
        "Date": "08.05.2017",
        "Close": 234.5
      },
      {
        "Date": "09.05.2017",
        "Close": 233.5
      },
      {
        "Date": "10.05.2017",
        "Close": 238.9
      },
      {
        "Date": "11.05.2017",
        "Close": 238.95
      },
      {
        "Date": "12.05.2017",
        "Close": 237
      },
      {
        "Date": "15.05.2017",
        "Close": 239
      },
      {
        "Date": "16.05.2017",
        "Close": 237.1
      },
      {
        "Date": "17.05.2017",
        "Close": 241
      },
      {
        "Date": "18.05.2017",
        "Close": 242.6
      },
      {
        "Date": "19.05.2017",
        "Close": 242.1
      },
      {
        "Date": "22.05.2017",
        "Close": 241.7
      },
      {
        "Date": "23.05.2017",
        "Close": 245.15
      },
      {
        "Date": "24.05.2017",
        "Close": 252.05
      },
      {
        "Date": "25.05.2017",
        "Close": 259.8
      },
      {
        "Date": "26.05.2017",
        "Close": 266.05
      },
      {
        "Date": "29.05.2017",
        "Close": 269.05
      },
      {
        "Date": "30.05.2017",
        "Close": 273
      },
      {
        "Date": "31.05.2017",
        "Close": 268.5
      },
      {
        "Date": "01.06.2017",
        "Close": 267.7
      },
      {
        "Date": "02.06.2017",
        "Close": 266.716
      },
      {
        "Date": "05.06.2017",
        "Close": 266.716
      },
      {
        "Date": "06.06.2017",
        "Close": 286.5
      },
      {
        "Date": "07.06.2017",
        "Close": 276.2
      }
    ]
    const chart = new Chart({
      container: 'coin-chart',
      autoFit: true,
      height: 500,
    });
    chart.scale({
      Date: {
        tickCount: 10
      },
      Close: {
        nice: true,
      }
    });
    chart.axis('Date', {
      label: {
        formatter: text => {
          const dataStrings = text.split('.');
          return dataStrings[2] + '-' + dataStrings[1] + '-' + dataStrings[0];
        }
      }
    });

    chart.line().position('Date*Close');
    // annotation
    const {min, max} = findMaxMin(data);
    chart.annotation().dataMarker({
      top: true,
      position: [max.Date, max.Close],
      text: {
        content: 'max：' + max.Close,
      },
      line: {
        length: 30,
      }
    });
    chart.annotation().dataMarker({
      top: true,
      position: [min.Date, min.Close],
      text: {
        content: 'min：' + min.Close,
      },
      line: {
        length: 50,
      }
    });
    chart.render();

  }
}
</script>

<style lang="scss" scoped>
.DetailsView {
  background: #F8F8F8;

  #coin-chart {
  }

  .DetailsView-content {
    width: 1000px;
    margin: 30px auto;
    display: flex;
    justify-content: space-between;

    .sub-title {
      display: flex;
      justify-content: space-between;
      margin: 30px 0;
      font-size: 1.3em;
      font-family: Roboto-Bold, Roboto;
      font-weight: bold;
      color: #2AAB00;
      line-height: 24px;

      .mangobox-button {
        padding: 6px 20px;
      }
    }

    .left-content, .right-content {
      width: 45%;

      .input-box {
        input {
          width: 320px;
          height: 36px;
          background: #F8F8F8;
          border-radius: 10px;
          border: 1px solid #D8D8D8;
        }
      }

      .row {
        justify-content: space-between;
        margin: 10px 0;

        .name {
          color: #999;
        }

        .value {
          font-weight: bold;
        }

      }
    }

    .right-content {
      .paid-list {
        .paid-item {
          padding: 10px 0;
          color: #CCCCCC;

          .paid-item-content {
            .value {
              color: #111;
              font-weight: bold;
            }
          }
        }
      }
    }
  }
  .create-token-dialog{
    z-index: 1000;
    text-align: center;
    position: fixed;
    width: 100vw;
    height: 100vh;
    margin-left: -50vw;
    top: 0px;
    left: 50%;
    will-change: opacity;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: opacity 0.1s ease-in-out 0s;
    .content {
      z-index: 2000;
      position: relative;
      background-color: rgb(255, 255, 255);
      width: 800px;
      padding: 50px;
      max-height: 600px;
      border-radius: 1em;
      margin: 10px;
      overflow: auto;
      input{
        width: 460px;
        height: 50px;
        background: #F8F8F8;
        border-radius: 10px;
        border: 1px solid #D8D8D8;
        margin: 20px 0;
        padding: 0 10px;
      }
      .operate-btns{
        display: flex;
        justify-content: center;

        .mangobox-button{
          padding: 10px 30px;
          margin-right: 30px;
        }
      }
    }
    .mask {
      width: 100%;
      height: 100%;
      position: fixed;
      z-index: 1000;
      background-color: rgba(0, 0, 0, 0.4);
    }
  }
}
</style>
