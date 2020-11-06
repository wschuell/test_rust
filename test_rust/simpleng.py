
import time
import numpy as np

start = time.time()

class NG(object):

	def __init__(self,N,tmax):
		self.N = N
		self.tmax = tmax
		self.next_word = 0
		self.t = 0
		self.pop = {i:[] for i in range(N)}

	def step(self):
		speaker_id,hearer_id = np.random.choice(range(N),size=(2,),replace=False)

		speaker = self.pop[speaker_id]
		hearer = self.pop[hearer_id]

		if len(speaker) == 0:
			speaker.append(self.next_word)
			self.next_word += 1
		word = np.random.choice(speaker)
		if word in hearer:
			self.pop[speaker_id] = [word]
			self.pop[hearer_id] = [word]
		else:
			hearer.append(word)

	def run(self):
		while self.t<self.tmax:
			self.step()
			self.t += 1
			if (self.t % self.N == 0) and (len(self.pop[0])==1) and all([len(ag)==1 and ag[0]==self.pop[0][0] for ag in self.pop.values()]):
				break



if __name__ == '__main__':
	N = 10**4
	tmax = 10**7

	ng = NG(N=N,tmax=tmax)
	ng.run()

	print(time.time()-start)
	print(ng.t)
