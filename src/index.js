var theCompaniesofDrinkingBlood = ["JD"];
var workdays = ["Mon","Tues","Wed","Thur","Fri","Sat"];
var offdays = ["Sun"];
const theWeek = offdays.concat(workdays);
class softwareEngineer 
{
    constructor(inaugurationCompany){
        this.theTimeofWork = 9;
        this.theTimeofOffWork = 21;
        this.theDaysWorkedperWeek = 7;
        this.alive = true;
        this.bodyStatus = "normal";// normal、sick
        this.workStatus = "working";// working、sleeping
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
                            this.workStatus="working";
                        }else{
                            if(Math.random()>0.001){
                                this.workStatus="working"; // overtime
                            }
                        }
                    }else{
                        if(Math.random()>0.001){
                            this.workStatus="working"; // overtime
                        }
                    }
                    if(Math.random()>0.3){
                        this.bodyStatus="sick";
                    }
                }else{
                    this.severityofDisease = "ICU";
                    if(Math.random()>0.1){
                        this.alive = false;
                    }
                }
            }
        }
    }
}