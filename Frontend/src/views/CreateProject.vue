<template>
  <div class="CreateProject daoContentBg">

    <div class="CreateProject-content">
      <h2>Creat your projects</h2>
      <div class="progress-box">
        <div class="progress-item" @click="activeStep=0">
          <div :class="activeStep>=0?'active':''" class="index">1</div>
          <div class="name">
            Project details
          </div>
        </div>
        <div class="progress-item" @click="activeStep=1">
          <div :class="activeStep>=1?'active':''" class="index">2</div>
          <div class="name">
            Funding cycle
          </div>
        </div>
        <div class="progress-item" @click="activeStep=2">
          <div :class="activeStep>=2?'active':''" class="index">3</div>
          <div class="name">
            Review and deploy
          </div>
        </div>
      </div>

      <div class="step-box" v-show="activeStep===0">
        <div class="input-box">
          <div class="name">
            Project name <strong style="color: #ff0000">*</strong>
          </div>
          <input type="text" v-model="ipfsObj.name"/>
        </div>
        <div class="input-box">
          <div class="name" >
            Project description
          </div>
          <textarea name="" v-model="ipfsObj.description" cols="50" rows="5"></textarea>
        </div>
        <div class="input-box">
          <div class="name">
            Logo
          </div>
          <input type="text" v-model="ipfsObj.icon"/>
        </div>
        <div class="input-box">
          <div class="name">
            Website
          </div>
          <input type="text" v-model="ipfsObj.webside"/>
        </div>
        <div class="input-box">
          <div class="name">
            Twitter handle
          </div>
          <input type="text" v-model="ipfsObj.twitter"/>
        </div>
        <div class="input-box">
          <div class="name">
            Discord
          </div>
          <input type="text" v-model="ipfsObj.discord"/>
        </div>
        <div class="input-box">
          <div class="name">
            Pay button text
          </div>
          <input type="text" v-model="ipfsObj.payButton"/>
        </div>
        <div class="input-box">
          <div class="name">
            Pay disclosure
          </div>
          <textarea name="" v-model="ipfsObj.payDisclosure" cols="50" rows="5"></textarea>
        </div>
        <div class="intro" style="opacity: 0.5;">
          Disclose any details to your contributors before they pay your project.
        </div>

        <button class="mangobox-button" @click="checkAndUpload">
          SAVE
        </button>
      </div>
      <div class="step-box" v-show="activeStep===1">
        <h3 class="step-item-title">
          01 Funding
        </h3>
        <div class="flex-box">
          <div class="switch">
            <div class="switch__1">
              <input id="switch-1" type="checkbox" v-model="autoFundingCycles">
              <label for="switch-1"></label>
            </div>
          </div>
          Automate funding cycles
        </div>
        <div class="input-box" v-show = "!autoFundingCycles">
          <div class="name">
            Funding cycle duration
          </div>
          <div class="flex-box">
            <input type="text" v-mode="durationDay">
            <el-select v-model="fundingCycleduration" placeholder="Days">
              <el-option
                  label="5"
                  value="0">
              </el-option>
            </el-select>
          </div>
        </div>
        <h2>
          Payouts
        </h2>
        <div class="intro">
          Choose how you would like to configure your payouts.
        </div>
        <div>
          <el-radio v-model="payouts" label="1">
            <strong>Amounts</strong>
            <div class="intro">
              <p>
                Distribute a specific amount of funds to entities each funding cycle.
              </p>
              <p>
                Your distribution limit will equal the sum of all payout amounts.
              </p>
            </div>
          </el-radio>
        </div>
        <div>
          <el-radio v-model="payouts" label="2">Percentages
            <div class="intro">
              <p>Distribute a percentage of all funds received to entities.</p>
              <p> Your distribution limit will be infinite.</p>
            </div>
          </el-radio>
        </div>
        <div class="add-payout-btn">
          Add payout
        </div>
        <h3 class="step-item-title">
          02 Project token
        </h3>
        <div class="input-box">
          <div class="name">
            Time interval between
          </div>
          <label>
            <input type="text" v-model="mustStartAtOrAfter"/>
          </label>
        </div>


        <div class="slider-box">
          <strong class="name">
            Initial mint rate
          </strong>

          <el-slider
              style="width: 500px"
              v-model="weight"
              show-input>
          </el-slider>
        </div>

        <div class="slider-box">
          <strong class="name">
            Discount Rate
          </strong>
          <el-slider
              style="width: 500px"
              v-model="discountRate"
              show-input>
          </el-slider>
        </div>


        <button class="mangobox-button" @click="activeStep++">
          SAVE
        </button>
      </div>
      <div class="step-box" v-show="activeStep===2">
        <h3 class="step-item-title">
          Project details
        </h3>
        <div class="intro">
          Project details can be edited at any time.
        </div>
        <div class="project-detail">
          <div class="detail-item">
            <div class="name">
              Name
            </div>
            <div class="value">
              {{ipfsObj.name?ipfsObj.name:'-'}}
            </div>
          </div>
          <div class="detail-item">
            <div class="name">
              Logo
            </div>
            <img :src="ipfsObj.icon" alt="">
          </div>
          <div class="detail-item">
            <div class="name">
              Webside
            </div>
            <div class="value">
              {{ipfsObj.webside?ipfsObj.webside:'-'}}
            </div>
          </div>
          <div class="detail-item">
            <div class="name">
              Pay button text
            </div>
            <div class="value">
              {{ipfsObj.payButton?ipfsObj.payButton:'-'}}
            </div>
          </div>
          <div class="detail-item">
            <div class="name">
              Twitter
            </div>
            <div class="value">
              {{ipfsObj.twitter?ipfsObj.twitter:'-'}}
            </div>
          </div>
          <div class="detail-item">
            <div class="name">
              Discord
            </div>
            <div class="value">
              {{ipfsObj.discord?ipfsObj.discord:'-'}}
            </div>
          </div>
          <div class="detail-item">
            <div class="name">
              Pay disclosure
            </div>
            <div class="value">
              {{ipfsObj.payDisclosure?ipfsObj.payDisclosure:'-'}}
            </div>
          </div>
        </div>

        <h3 class="step-item-title">
          Funding cycle details
        </h3>
        <div class="intro">
          Once launched, your first funding cycle can't be changed. You can reconfigure upcoming
          funding cycles according to the project's reconfiguration rules.
        </div>
        <div class="funding-cycle-detail">
          <div class="detail-item">
            <div class="name">
              Duration
            </div>
            <div class="value">
              {{durationDay?durationDay:'auto'}}
            </div>
          </div>
          <div class="detail-item">
            <div class="name">
              Initial mint rate
            </div>
            <div class="value">
              {{weight?weight:'auto'}}
            </div>
          </div>
          <div class="detail-item">
            <div class="name">
              Discount Rate
            </div>
            <div class="value">
              {{discountRate?discountRate:'0'}}
            </div>
          </div>
          <div class="detail-item">
            <div class="name">
              Start After
            </div>
            <div class="value">
              {{mustStartAtOrAfter?mustStartAtOrAfter:'0'}}
            </div>
          </div>
        </div>

        <button class="mangobox-button"  @click="launchProjectFor">
          DEPLOY
        </button>
      </div>

    </div>

  </div>
</template>

<script>

import { uploadJson} from "../utils/ipfsApi";

export default {
  name: "CreateProject",
  data() {
    return {
      //form
      ipfsObj:{

      },
      //end form
      reservedTokens:0,
      payouts: 0,
      fundingCycleduration: null,
      activeStep:1,
      ipfsStr:"",//uploadipfs json address
      autoFundingCycles:false,
      durationDay:0,
      mustStartAtOrAfter:undefined,
      weight:0,
      discountRate:0,

    }
  },

  methods:{
    async checkAndUpload(){

      if(!this.ipfsObj.name){
        this.$eventBus.$emit('message', {
          message: "Please Input Name",
          type: "error"
        })
        return
      }

      this.activeStep++
    },

    mint(){
      this.$store.dispatch("MBProjects/mint")
    },
    ownerOf(){
      this.$store.dispatch("MBProjects/ownerOf")
    },
    async launchProjectFor(){
      this.$store.commit("app/SET_LOADINDING",true)
      const res = await uploadJson({
        ...this.ipfsObj,
        createTime: new Date()
      })
      this.ipfsStr = res.data.IpfsHash
      console.log(this.ipfsStr )
      this.$store.dispatch("MBController/launchProjectFor",{
        _owner:this.$store.state.app.account,
        _projectMetadata:this.ipfsStr,
        _data:{
          duration: 0,
          weight: 0,
          discountRate: 0,
          ballot:this.$store.state.app.account
        },
        _metadata:0,
        _mustStartAtOrAfter: this.mustStartAtOrAfter,
        _groupedSplits:[],
        _fundAccessConstraints: [{
          terminal:this.$store.state.app.account,
          token:this.$store.state.app.account,
          distributionLimit:0,
          distributionLimitCurrency:0,
          overflowAllowance:0,
          overflowAllowanceCurrency:0
        }],
        _terminals: ["5Gx2WFYjXC8yv5z2hSpEASobNVQ45d5gccGjRd6AgPMM7qqC"],
        _memo:""
      }).then(()=>{
        this.$store.commit("app/SET_LOADINDING",false)
      }).catch(()=>{
        this.$store.commit("app/SET_LOADINDING",false)
      })

    }
  },
  created() {
    this.$eventBus.$on('message', (message) => {
      if(message.type == 'success'){
        this.$router.push('/')
      }
    })
  },
  beforeDestroy() {
    this.$eventBus.$on('message', ()=>{})
  }
}
</script>

<style lang="scss" scoped>
.CreateProject-content {
  width: 1200px;
  margin: 30px auto;
  .intro{
    opacity: 0.8;
    margin: 10px 0;
  }
  .slider-box{
    padding: 10px 0;
  }
  .step-item-title {
    font-size: 2em;
    font-weight: bold;
    margin: 20px 0;
  }

  .mangobox-button {
    padding: 8px 20px;
    margin: 30px 0;
    min-width: 200px;
  }

  .progress-box {
    display: flex;

    .progress-item {
      display: flex;
      margin-right: 30px;
      align-items: center;
      cursor: pointer;
      .index {
        width: 30px;
        height: 30px;
        background: #D8D8D8;
        border: 1px solid #C0C0C0;
        border-radius: 50%;
        text-align: center;
        line-height: 30px;
        font-size: 18px;
        font-family: Roboto-Bold, Roboto;
        font-weight: bold;
        color: #333333;

        &.active {
          color: #fff;
          background: linear-gradient(225deg, #54D500 0%, #2AAA00 100%);
        }
      }

      .name {
        margin-left: 10px;
      }
    }
  }

  .input-box {
    padding: 10px 0;
    .intro{
      width: 460px;

    }
    .name {
      line-height: 40px;
      font-size: 18px;
      font-weight: bold;
      font-family: Roboto-Bold, Roboto;
    }

    ::v-deep {
      .el-input__inner {
        margin-left: 20px;
        width: 120px !important;
        height: 40px;
        background: #F8F8F8;
        border-radius: 10px;
        border: 1px solid #D8D8D8;
        padding: 0 10px;
      }
    }

    input, textarea {
      width: 460px;
      height: 40px;
      background: #F8F8F8;
      border-radius: 10px;
      border: 1px solid #D8D8D8;
      padding: 0 10px;
    }

    textarea {
      height: auto;
      padding: 10px;
    }
  }
  .add-payout-btn{
    width: 640px;
    height: 50px;
    background: #F8F8F8;
    border-radius: 10px;
    text-align: center;
    line-height: 50px;
    color: #52D200;
    font-weight: bold;
    cursor: pointer;
  }
  .radio-box {
    display: flex;
    align-items: center;

    input {
      width: 3em;
      height: 1.5em;
    }

    p {
      width: 500px;
    }
  }

  .funding-cycle-detail,.project-detail{
    display: flex;
    flex-wrap: wrap;
    .detail-item{
      margin-right: 80px;
      .name{
        font-size: 18px;
        font-family: Roboto-Medium, Roboto;
        font-weight: bold;
        color: #2AAB00;
        line-height: 50px;
      }
      img{
        width: 50px;
        height: 50px;
      }
    }
  }
}
</style>
