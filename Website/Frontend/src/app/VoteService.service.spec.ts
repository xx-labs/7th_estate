/* tslint:disable:no-unused-variable */

import { TestBed, async, inject } from '@angular/core/testing';
import { VoteService } from './VoteService.service';

describe('Service: VoteService', () => {
  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [VoteService]
    });
  });

  it('should ...', inject([VoteService], (service: VoteService) => {
    expect(service).toBeTruthy();
  }));
});
