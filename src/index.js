var theCompaniesofDrinkingBlood = ["JD"];
var workdays = ["Mon","Tues","Wed","Thur","Fri","Sat"];
var offdays = ["Sun"];
const theWeek = offdays.concat(workdays);
const overtimeRate = 0.999;
const sickRate = 0.7;
const deadRate = 0.9;
class softwareEngineer 
{
    constructor(inaugurationCompany){
        this.theTimeofWork = 9;
        this.theTimeofOffWork = 21;
        this.theDaysWorkedperWeek = 7;
        this.alive = true;
        this.bodyStatus = {
            NORMAL:"normal",
            SICK:"sick"
        };// normal、sick
        this.workStatus = {
            WORKING:"working",
            SLEEPING:"sleeping"
        };// working、sleeping
        this.inaugurationCompany = inaugurationCompany;
        this.severityofDisease = null;
    }
    work()
    {
        if(theCompaniesofDrinkingBlood.includes(this.inaugurationCompany)){
            let currentDate=new Date();
            while(this.alive){
                if(this.bodyStatus!=="sick"){
                    if(workdays.includes(theWeek[currentDate.getDay()])){
                        if(currentDate.getHours()>=this.theTimeofWork&&currentDate.getHours()<=this.theTimeofOffWork){
                            this.workStatus=workStatus.WORKING;
                        }else{
                            if(Math.random()<overtimeRate){
                                this.workStatus=workStatus.WORKING; // overtime
                            }else{
                                this.workStatus=workStatus.SLEEPING;
                            }
                        }
                    }else{
                        if(Math.random()<overtimeRate){
                            this.workStatus=workStatus.WORKING; // overtime
                        }else{
                            this.workStatus=workStatus.SLEEPING;
                        }
                    }
                    if(Math.random()>sickRate){
                        this.bodyStatus=bodyStatus.SICK;
                    }
                }else{
                    this.severityofDisease = "ICU";
                    if(Math.random()<deadRate){
                        this.alive = false;
                    }
                }
            }
        }
    }
    addtheCompaniesofDrinkingBlood(companyName){
        this.theCompaniesofDrinkingBlood.push(companyName);
    }
}