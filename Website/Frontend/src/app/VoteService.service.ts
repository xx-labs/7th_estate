import { Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";


@Injectable({
  providedIn: 'root'
})
export class VoteService {
  url = "/vote";
  constructor(private http: HttpClient) { }

  postVote(votecode: string, csrf: string) {
    console.log(this.url);
    return this.http.post(this.url, { votecode: votecode, _csrf: csrf });
  }

  getCsrf() {
    return this.http.get(this.url + '/csrf');
  }

  checkReceipt(hash: string) {
    return this.http.get(this.url + '/status/' + hash);
  }

}
